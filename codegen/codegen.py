# Build the Rust field and message types from the FIX spec.

import errno
import os
import re
from collections import namedtuple, Counter
import xml.etree.ElementTree as ET

from codegen_base import *


def format_name(name):
    # Handle case where name begins with a numeric character
    numbers = '0123456789'
    if name[0] in numbers:
        num_end_index = 1
        while num_end_index < len(name) and name[num_end_index] in numbers:
            num_end_index += 1

        word = ENGINE.number_to_words(int(name[0:num_end_index]))
        name = word + name[num_end_index:]
    name = re.sub(' ', '', name)
    name = re.sub('-', '', name)
    return name


def format_type_name(name):
    """UPPER_SNAKE_CASE -> UpperCamelCase"""
    name = format_name(name)
    return ''.join(word.lower().title() for word in name.split('_'))


def format_struct_field_name(name):
    """UpperCamel-Case -> lower_snake_case"""
    name = re.sub(' |\\-', '_', name)
    name = format_name(name)

    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    s2 = re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()

    return format_reserved_name(s2)


class EnumMember(namedtuple("EnumMember", ["name", "code"])):
    """
    Represents one member of a Rust enum type (i.e. one value of a FIX enum
    field).
    """
    def __str__(self):
        if len(self.code) == 1:
            return "    {n} = b'{c}' as isize,".format(n=self.name, c=self.code)
        else:
            return '    {n},'.format(n=self.name)

    def match_arm(self, typename):
        """
        Line in the match statement that converts from bytes to this enum
        value.
        """
        matchstr = tab('b"{c}" => Some({a}::{n}),', 3)
        return matchstr.format(c=self.code, a=typename, n=self.name)


class Field(object):

    def __init__(self, field, version):
        self.raw_name = field.get('name')
        self.name = format_name(self.raw_name)
        self.number = field.get('number')
        self.enums = None
        self.base_type = None

        children = field.getchildren()
        if len(children) == 0:
            self.base_type = fix_types(field.get("type"), version)
        else:
            self.enums = [EnumMember(format_type_name(c.get('description')),
                               c.get('enum'))
                          for c in children]
            self.multi_byte_code = any( len(e.code) > 1 for e in self.enums )

            # sort to avoid conflicting discriminants
            if self.multi_byte_code:
                self.enums.sort(key=lambda e: (len(e.code)>1, e.code, e.name))

        self.derive = "#[derive(Debug, Clone, Copy, PartialEq, Eq)]"
        return

    def __eq__(self, other):
        sort_enums = lambda e: sorted([] if e is None else e)

        return self.name == other.name and\
                self.raw_name == other.raw_name and\
                sort_enums(self.enums) == sort_enums(other.enums) and\
                self.base_type == other.base_type

    def __str__(self):
        if self.enums is not None:
            enums = "\n".join( str(e) for e in self.enums)
            typestr = "pub enum {a} {{\n{b}\n}}\n".format(a=self.name, b=enums)

            from_bytes = "\n".join(e.match_arm(self.name) for e in self.enums)

            push_1_byte_code = 'v.push(*self as u8)'
            if not self.multi_byte_code:
                to_bytes = '        {0};'.format(push_1_byte_code)
            else:
                to_bytes = '        match *self {\n'
                for e in self.enums:
                    if len(e.code) > 1:
                        s = '            {a}::{n} => v.extend(b"{b}"),\n'
                        to_bytes += s.format(a=self.name, n=e.name, b=e.code)
                if any( len(e.code) == 1 for e in self.enums):
                    to_bytes += '            _ => {0}\n'.format(push_1_byte_code)
                to_bytes += '        }'

            implstr = \
'''impl FIXValue for {a} {{
    fn from_bytes(bytes: &[u8]) -> Option<{a}> {{
        match bytes {{
{b}
            _ => None
        }}
    }}

    fn to_bytes(&self, mut v: &mut Vec<u8>) {{
{t}
    }}
}}
'''.format(a=self.name, b=from_bytes, t=to_bytes)
            return "\n".join((self.derive, typestr, implstr))

        # don't generate type aliases for non-enum types
        return ""


class OptionalField(namedtuple("OptionalField", ["field", "is_option"])):

    def to_str(self, fieldname, typename):
        if self.is_option:
            return "{a}: Option<{b}>".format(a=fieldname, b=typename)
        else:
            return "{a}: {b}".format(a=fieldname, b=typename)

    def __str__(self):
        if isinstance(self.field, Field) and self.field.base_type is not None:
            typename = self.field.base_type
        else:
            typename = self.field.name
        fieldname = format_struct_field_name(self.field.name)
        return self.to_str(fieldname, typename)


class Component(object):

    def __init__(self, comp, tagdict, name=None):
        if name is None:
            name = comp.get('name')
        self.name = format_name(name)

        self.fields = []
        for f in get_fields(comp):
            field = tagdict[f.get('name')]
            optional = f.get('required') == 'N'
            self.fields.append(OptionalField(field, optional))
        return

    def __eq__(self, other):
        return self.name == other.name and self.fields == other.fields

    def __str__(self):
        field_str = ",\n".join( "    pub " + str(f) for f in self.fields )
        return "pub struct {a} {{\n{b}\n}}\n".format(a=self.name, b=field_str)


def make_comps(comps, tags):
    new_components = []
    while True:
        if len(comps) == 0:
            return new_components

        for i, comp in enumerate(comps):
            if all( (f.get('name') in tags) for f in get_fields(comp) ):
                new_component = Component(comp, tags)
                new_components.append(new_component)
                tags[comp.get('name')] = new_component
                break
        comps.pop(i)
    raise ValueError("Recursive component dependency")


class Message(namedtuple("Message", ["name", "code", "fields"])):

    def __str__(self):
        doc = tab("/// {a} message. Message code: {c}")
        doc = doc.format(a=self.name, c=self.code)

        if len(self.fields) == 0:
            return "{d}    {a}".format(d=doc, a=self.name)
        else:
            fields = ",\n".join( " " * 8 + str(f) for f in self.fields )
            struct_str = "{d}\n    {a} {{\n{b}\n    }}"
            return struct_str.format(d=doc, a=self.name, b=fields)


class MessageBody(object):

    def __init__(self, messages, tagdict):
        self.messages = []
        for msg in messages.getchildren():
            fields = []
            for f in get_fields(msg):
                field = tagdict[f.get('name')]
                optional = f.get('required') == 'N'
                fields.append(OptionalField(field, optional))

            msg_name = msg.get('name')
            msg_code = msg.get('msgtype')
            self.messages.append(Message(msg_name, msg_code, fields))
        return

    def __str__(self):
        messages=",\n\n".join( str(m) for m in self.messages )
        body = ['pub struct Message {',
            tab('/// Standard message header.'),
            tab('pub header: Header,'),
            tab('/// Message body'),
            tab('pub body: MessageBody,'),
            '}\n',
            'pub enum MessageBody {{\n{0}\n}}'.format(messages),]
        return "\n".join(body)


def set_up_directory(src_path):
    """
    Set up directory (e.g. 'src/fix41') for generated code.
    """
    try:
        os.mkdir(src_path);
    except OSError as exception:
        if exception.errno != errno.EEXIST:
            raise
    return


def make_mod_rs(src_path):
    """
    Make the mod.rs file for a source diectory.
    """
    with open("{0}/mod.rs".format(src_path), "wb") as f:
        f.write("mod fields;\n")
        f.write("mod message;\n\n")
        f.write("pub use self::fields::*;\n")
        f.write("pub use self::message::*;\n")
    return


def make_fields_rs(src_path, fields, components, common_set=None):
    common_types = []
    unique_fields = []
    unique_components = []

    if common_set is not None:
        for f in fields:
            (common_types if f.name in common_set else unique_fields).append(f)

        for c in components:
            (common_types if f.name in common_set else unique_components).append(c)
    else:
        unique_fields = fields
        unique_components = components

    common_str = "pub use common::{0} as {0};"
    common = sorted(common_str.format(c.name) for c in common_types)

    fields = sorted( str(f) for f in unique_fields if str(f) != "" )
    components = sorted( str(c) for c in unique_components )

    with open("{0}/fields.rs".format(src_path), "wb") as f:
        if len(unique_components) > 0:
            f.write("use types::*;\n")
        f.write("use protocol::FIXValue;\n")
        f.write("\n".join(common))
        f.write("\n\n\n");
        f.write("\n".join(fields))
        f.write("\n")
        f.write("\n".join(components))
    return


def make_message_rs(src_path, version, head, messages):
    with open("{0}/message.rs".format(src_path), "wb") as f:
        f.write("use types::*;\n")
        f.write("use {0}::fields::*;\n\n".format(version.lower()))
        f.write(str(head) + "\n")
        f.write(str(messages))
    return


class CommonCodegen(object):
    """
    Object representing a collection of Fields and Components that are present
    in all FIX protocol versions.
    """
    def __init__(self, codegens, src_path):
        self.src_path = src_path

        noncommon = set()  # names of non-universal items
        common = {}        # name -> value map of universal items
        counts = Counter() # counts of universal items

        for codegen in codegens:
            for name, item in codegen.tags.iteritems():
                if name not in noncommon:
                    if name not in common:
                        counts[name] += 1
                        common[name] = item
                    elif common[name] == item:
                        counts[name] += 1
                    else:
                        del counts[name]
                        del common[name]
                        noncommon.add(name)

        self.fields = []
        self.components = []
        self.name_set = set()
        for name, item in common.iteritems():
            if counts[name] > 1:
                if isinstance(item, Field):
                    if str(item) == "":
                        continue
                    self.fields.append(item)
                elif isinstance(item, Component):
                    self.components.append(item)
                else:
                    raise ValueError
                self.name_set.add(name)
        return

    def write(self):
        set_up_directory(self.src_path)

        # Minimal mod.rs
        with open("{0}/mod.rs".format(self.src_path), "wb") as f:
            f.write("mod fields;\n")
            f.write("pub use self::fields::*;\n")

        make_fields_rs(self.src_path, self.fields, self.components)
        return


class Codegen(object):
    """
    Object that reads the spec for a FIX protocol version and generates Rust
    code.

    Attributes:
        version - protocol version name
        spec_path - path to the spec file
        src_path - path to the source directory to be generated
        fields - list of Fields
        comps - list of Components
        tags - dictionary mapping names (tags) to Fields/Components
        header - Component representing the std. message header
        messages - MessageBody representing the enum of possible messages
    """
    def __init__(self, version, spec_dir, src_dir):
        self.version = version
        self.spec_path = "{d}/{v}.xml".format(d=spec_dir, v=version)
        self.src_path = "{d}/{v}".format(d=src_dir, v=version.lower())

        spec = ET.parse(self.spec_path).getroot()
        assert get_version_name(spec) == version, "Name mismatch " + version

        header, msgs, _, components, fields = spec.getchildren()

        self.fields = [Field(f, version) for f in fields.getchildren()]
        self.tags = { f.raw_name : f for f in self.fields }
        self.comps = make_comps(components.getchildren(), self.tags)

        self.header = Component(header, self.tags, "Header")
        self.messages = MessageBody(msgs, self.tags)
        return

    def write(self, common):
        """
        Generate the Rust source and write it to file.
        """
        set_up_directory(self.src_path)
        make_mod_rs(self.src_path)
        make_fields_rs(self.src_path, self.fields, self.comps, common.name_set)
        make_message_rs(self.src_path, self.version, self.header, self.messages)
        return


def main():
    sources = [Codegen(version, "spec", "src") for version in VERSIONS]

    common = CommonCodegen(sources, "src/common")
    common.write()

    for source in sources:
        source.write(common)
    return


if __name__ == '__main__':
    main()

# Build the Rust field and message types from the FIX spec.

import errno
import os
import re
from collections import namedtuple
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


class Enum(namedtuple("Enum", ["name", "code"])):

    def __str__(self):
        if len(self.code) == 1:
            return "    {n} = b'{c}' as isize,".format(n=self.name, c=self.code)
        else:
            return '    {n},'.format(n=self.name)

    def match_arm(self, typename):
        """
        Part of the match statement that converts from bytes to this enum
        value.
        """
        matchstr = '            b"{c}" => Some({a}::{n}),'
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
            self.enums = [Enum(format_type_name(c.get('description')),
                               c.get('enum'))
                          for c in children]
            self.multi_byte_code = any( len(e.code) > 1 for e in self.enums )

            # sort to avoid conflicting discriminants
            if self.multi_byte_code:
                self.enums.sort(key=lambda e: (len(e.code)>1, e.code, e.name))

        self.derive = "#[derive(Debug, Clone, Copy, PartialEq, Eq)]"
        return

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
    special_fields = ["CheckSum"]

    def handle_special_field(self, typename):
        """Individual special fields whose type needs to be modified"""
        if self.field.name == "CheckSum":
            fieldname = format_struct_field_name(self.field.name)
            typename = "u8"
        return self.to_str(fieldname, typename)

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

        if self.field.name in OptionalField.special_fields:
            return self.handle_special_field(typename)
        else:
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

        tagdict.update({ name: self })
        return

    def __str__(self):
        field_str = ",\n".join( "    pub " + str(f) for f in self.fields )
        return "pub struct {a} {{\n{b}\n}}\n".format(a=self.name, b=field_str)


class Message(namedtuple("Message", ["name", "code", "fields"])):

    def handle_special_field_groups(self):
        for field in self.fields:
            pass

    def __str__(self):
        doc = "    /// {a} message. Message code: {c}\n"
        doc = doc.format(a=self.name, c=self.code)

        if len(self.fields) == 0:
            return "{d}    {a}".format(d=doc, a=self.name)
        else:
            # there are some special fields (raw data, signature) tha need to
            # be modified before we convert each field to string
            self.handle_special_field_groups()

            fields = ",\n".join( " " * 8 + str(f) for f in self.fields )
            struct_str = "{d}    {a} {{\n{b}\n    }}"
            return struct_str.format(d=doc, a=self.name, b=fields)


class MessageBody(object):

    def __init__(self, messages, tagdict):
        self.messages = []
        self.field_contents = []
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
            tab('/// Standard message header.', 1),
            tab('pub header: Header,', 1),
            tab('/// Message body', 1),
            tab('pub body: MessageBody,', 1),
            '}\n',
            'pub enum MessageBody {{\n{0}\n}}'.format(messages),]
        return "\n".join(body)


def set_up_directory(src_path):
    """
    Set up the directory (e.g. 'src/fix41') for the generated code and create
    the mod.rs file.
    """
    try:
        os.mkdir(src_path);
    except OSError as exception:
        if exception.errno != errno.EEXIST:
            raise

    # make mod.rs
    with open("{0}/mod.rs".format(src_path), "wb") as f:
        f.write("mod fields;\n")
        f.write("mod message;\n\n")
        f.write("pub use self::fields::*;\n")
        f.write("pub use self::message::*;\n")
    return


def make_fields_rs(src_path, fields, components):
    fields = sorted( str(f) for f in fields if str(f) != "" )
    components = sorted( str(c) for c in components )
    with open("{0}/fields.rs".format(src_path), "wb") as f:
        if len(components) > 0:
            f.write("use types::*;\n")
        f.write("use protocol::FIXValue;\n\n")
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


def codegen(verison, spec_dir, src_dir):
    spec_path = "{d}/{v}.xml".format(d=spec_dir, v=version)
    src_path = "{d}/{v}".format(d=src_dir, v=version.lower())

    spec = ET.parse(spec_path).getroot()
    assert get_version_name(spec) == version

    header, msgs, _, components, fields = spec.getchildren()

    tagfields = [Field(f, version) for f in fields.getchildren()]
    tagdict = { f.raw_name : f for f in tagfields }

    comps = [Component(c, tagdict) for c in components.getchildren()]
    #tagdict.update({ c.name : c for c in comps })

    head = Component(header, tagdict, "Header")
    messages = MessageBody(msgs, tagdict)

    set_up_directory(src_path)
    make_fields_rs(src_path, tagfields, comps)
    make_message_rs(src_path, version, head, messages)
    return


if __name__ == '__main__':
    for version in VERSIONS:
        print "Generating Rust code for {0}".format(version)
        codegen(version, "spec", "src")

# Build the Rust field and message types from the FIX spec.

import errno
import os
import re
from collections import namedtuple
import xml.etree.ElementTree as ET

import inflect


ENGINE = inflect.engine()

SPEC_VERSIONS = ["FIX40", "FIX41", "FIX42", "FIX43", "FIX44",
                 "FIX50", "FIX50SP1", "FIX50SP2", "FIXT11"]

FIX_TYPES = {'AMT': 'Amt',
            'BOOLEAN': 'FIXBoolean',
            'CHAR': 'FIXChar',
            'CURRENCY': 'Currency',
            'DATA': 'Data',
            'DATE': 'UTCDateOnly',
            'DAYOFMONTH': 'DayOfMonth',
            'EXCHANGE': 'FIXString',
            'FLOAT': 'FIXFloat',
            'INT': 'FIXInt',
            'LENGTH': 'Length',
            'LOCALMKTDATE': 'FIXString',
            'MONTHYEAR': 'MonthYear',
            'MULTIPLEVALUESTRING': 'MultipleValueString',
            'NUMINGROUP': 'NumInGroup',
            'PERCENTAGE': 'Percentage',
            'PRICE': 'Price',
            'PRICEOFFSET': 'PriceOffset',
            'QTY': 'Qty',
            'SEQNUM': 'SeqNum',
            'STRING': 'FIXString',
            'TENOR': 'Tenor',
            'TIME': 'UTCTimeOnly',
            'UTCTIMEONLY': 'UTCTimeOnly',
            'UTCTIMESTAMP': 'UTCTimestamp',
            'UTCDATE': 'UTCDateOnly',
            'XMLDATA': 'XmlData'}


def format_name(name):
    # Handle case where name begins with a numeric character
    numbers = '0123456789'
    if name[0] in numbers:
        tokens = name.split("_")

        num_word = ENGINE.number_to_words(int(tokens[0]))
        num_word = re.sub(",", "", num_word)
        num_word = re.sub("-", " ", num_word)

        tokens[0] = "".join(num_word.split()).upper()
        name = "_".join(tokens)
    return name


def format_type_name(name):
    """UPPER_SNAKE_CASE -> UpperCamelCase"""
    name = format_name(name)
    return "".join(word.lower().title() for word in name.split("_"))


def format_struct_field_name(name):
    """UpperCamelCase -> lower_snake_case"""
    name = format_name(name)
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()


class Enum(namedtuple("Enum", ["name", "code"])):

    def __str__(self):
        if len(self.code) == 1:
            return "    {n} = b'{c}' as isize,".format(n=self.name, c=self.code)
        else:
            return "    {n},".format(n=self.name)

    def match_arm(self, typename):
        matchstr = '            b"{c}" => Some({a}::{n}),'
        return matchstr.format(c=self.code, a=typename, n=self.name)


class Field(object):

    def __init__(self, field):
        self.raw_name = field.get('name')
        self.name = format_name(self.raw_name)
        self.number = field.get('number')
        self.enums = None
        self.base_type = None

        children = field.getchildren()
        if len(children) == 0:
            self.base_type = FIX_TYPES[field.get("type")]
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
'''impl FIXField for {a} {{
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

    def __str__(self):
        if self.field.base_type is not None:
            typename = self.field.base_type
        else:
            typename = self.field.name

        fieldname = format_struct_field_name(self.field.name)

        if self.is_option:
            return "{a}: Option<{b}>".format(a=fieldname, b=typename)
        else:
            return "{a}: {b}".format(a=fieldname, b=typename)


class HeadTail(object):

    def __init__(self, header, tagdict, name):
        self.name = name

        self.fields = []
        for f in header.getchildren():
            field = tagdict[f.get('name')]
            optional = f.get('required') == 'N'
            self.fields.append(OptionalField(field, optional))
        return

    def __str__(self):
        field_str = ",\n".join( "    pub " + str(f) for f in self.fields )
        return "pub struct {a} {{\n{b}\n}}\n".format(a=self.name, b=field_str)


class Message(namedtuple("Message", ["name", "code", "fields"])):

    def __str__(self):
        fields = ",\n".join( " " * 8 + str(f) for f in self.fields )
        return "    {a} {{\n{b}\n    }}".format(a=self.name, b=fields)


class MessageBody(object):

    def __init__(self, messages, tagdict):
        self.messages = []
        self.field_contents = []
        for msg in messages.getchildren():
            fields = []
            for f in msg.getchildren():
                field = tagdict[f.get('name')]
                optional = f.get('required') == 'N'
                fields.append(OptionalField(field, optional))

            msg_name = msg.get('name')
            msg_code = msg.get('msgtype')
            self.messages.append(Message(msg_name, msg_code, fields))
        return

    def __str__(self):
        messages=",\n".join( str(m) for m in self.messages )
        body = "pub enum MessageBody {{\n{0}\n}}".format(messages)
        msg = \
'''pub struct Message {
    pub header: Header,
    pub trailer: Trailer,
    pub body: MessageBody
}
'''
        return "\n".join((msg, body))


def set_up_directory(src_path):
    try:
        os.mkdir(src_path);
    except OSError as exception:
        if exception.errno != errno.EEXIST:
            raise

    # make mod.rs
    with open("{0}/mod.rs".format(src_path), "wb") as f:
        f.write("mod fields;\n")
        f.write("mod message;\n")
        f.write("\n")
        f.write("pub use self::fields::*;\n")
        f.write("pub use self::message::*;\n")


def make_fields_rs(src_path, fields):
    fields = sorted( str(f) for f in fields if str(f) != "" )
    with open("{0}/fields.rs".format(src_path), "wb") as f:
        f.write("use types::FIXField;\n\n")
        f.write("\n".join(fields))
    return


def make_message_rs(src_path, version, head, tail, messages):
    with open("{0}/message.rs".format(src_path), "wb") as f:
        f.write("use types::*;\n")
        f.write("use {0}::fields::*;\n\n".format(version))
        f.write(str(head) + "\n")
        f.write(str(tail) + "\n")
        f.write(str(messages))
    return


def get_version_name(spec):
    maj_version = spec.get('major')
    min_version = spec.get('minor')
    servicepack = spec.get('servicepack')

    version_str = 'FIX' + maj_version + '.' + min_version
    if servicepack != '0':
        version_str += 'SP' + servicepack

    return version_str


def codegen(verison, spec_path, src_path):
    spec = ET.parse(spec_path).getroot()
    header, msgs, trailer, components, fields = spec.getchildren()

    tagfields = [Field(f) for f in fields.getchildren()]
    tagdict = { f.raw_name : f for f in tagfields }

    head = HeadTail(header, tagdict, "Header")
    tail = HeadTail(header, tagdict, "Trailer")
    messages = MessageBody(msgs, tagdict)

    set_up_directory(src_path)
    make_fields_rs(src_path, tagfields)
    make_message_rs(src_path, version, head, tail, messages)
    return


if __name__ == '__main__':
    spec_files = ["spec/{0}.xml".format(s) for s in SPEC_VERSIONS]
    src_paths = ["src/{0}".format(s) for s in SPEC_VERSIONS]
    version_names = [v.lower() for v in SPEC_VERSIONS]
    versions = zip(version_names, spec_files, src_paths)

    for version, spec_path, src_path in versions:
        print version
        codegen(version, spec_path, src_path.lower())

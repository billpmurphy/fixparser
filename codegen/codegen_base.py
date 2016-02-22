import inflect


###############################################################################
# FIX enumerations/utils


VERSIONS = ['FIX40',
            'FIX41',
            'FIX42',
            'FIX43',
            'FIX44',
            'FIX50',
            'FIX50SP1',
            'FIX50SP2',
            'FIXT11']

DATA_LEN_FIELDS = set([
    'EncodedAllocTextLen',
    'EncodedIssuerLen',
    'EncodedHeadlineLen',
    'EncodedListExecInstLen',
    'EncodedSecurityDescLen',
    'EncodedTextLen',
    'EncodedUnderlyingIssuerLen',
    'EncodedUnderlyingSecurityDescLen',
    'RawDataLen',
    'SignatureLen',
    'XmlDataLen'])


FIX_TYPES = {'AMT': 'Amt',
            'BOOLEAN': 'FIXBoolean',
            'CHAR': None,
            'COUNTRY': 'Country',
            'CURRENCY': 'Currency',
            'DATA': 'Data',
            'DATE': 'UTCDateOnly',
            'DAYOFMONTH': 'DayOfMonth',
            'EXCHANGE': None,
            'FLOAT': 'FIXFloat',
            'INT': 'FIXInt',
            'LENGTH': 'Length',
            'LOCALMKTDATE': 'LocalMktDate',
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
            'UTCDATEONLY': 'UTCDateOnly',
            'UTCTIMEONLY': 'UTCTimeOnly',
            'UTCTIMESTAMP': 'UTCTimestamp',
            'UTCDATE': 'UTCDateOnly',
            'XMLDATA': 'XmlData'}


def fix_types(typename, version):
    """
    Map a FIX schema type name to its Rust equivalent.
    """
    if typename == 'CHAR':
        if version in ('FIX40', 'FIX41'):
            return 'FIXString'
        else:
            return 'FIXChar'
    elif typename == 'EXCHANGE':
        if version in ('FIX40', 'FIX41', 'FIX42'):
            return 'ReutersExchange'
        else:
            return 'MICExchange'
    else:
        return FIX_TYPES[typename]


###############################################################################
# Formatting utils


ENGINE = inflect.engine()

RESERVED_WORDS = set(['yield', 'pub', 'use', 'mod'])


def format_reserved_name(name):
    """
    Convert a reserved word to a name that can safely be used as a Rust field
    name.
    E.g. "yield" -> "yield_"
    """
    if name in RESERVED_WORDS:
        name = name + "_"
    return name


def tab(text, n):
    """
    Indent some generated code by `n` 4-space indents.
    """
    lines = text.split('\n')
    lines = ['    ' + line for line in lines]
    return '\n'.join(lines)


###############################################################################
# XML spec utils


def get_version_name(spec):
    """
    FIX XML spec -> str version name of spec
    """
    maj_version = spec.get('major')
    min_version = spec.get('minor')
    servicepack = spec.get('servicepack')

    version_str = 'FIX' + maj_version + min_version
    if servicepack != '0':
        version_str += 'SP' + servicepack

    return version_str


def get_fields(message):
    """
    Extract all <field> elements from a <component> or <message> in the spec.
    Handles special fields like `data`.
    """
    fields = []
    for field in message.getchildren():
        if field.tag in ('field', 'component'):
            # Raw data length fields
            if field.tag == 'field' and field.get('name') in DATA_LEN_FIELDS:
                continue

            # Regular fields
            fields.append(field)

        elif field.tag == 'group':
            fields.extend(get_fields(field))

        else:
            raise ValueError(field.tag)
    return fields

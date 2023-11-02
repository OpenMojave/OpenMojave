use super::RecordType;

pub mod actor_values;
pub mod model;
pub mod object_bounds;
pub mod xnam;

pub const HEDR: RecordType = RecordType::from_value(b"HEDR");
pub const MAST: RecordType = RecordType::from_value(b"MAST");
pub const OFST: RecordType = RecordType::from_value(b"OFST");
pub const DELE: RecordType = RecordType::from_value(b"DELE");
pub const NAME: RecordType = RecordType::from_value(b"NAME");
pub const CNAM: RecordType = RecordType::from_value(b"CNAM");
pub const SNAM: RecordType = RecordType::from_value(b"SNAM");
pub const ONAM: RecordType = RecordType::from_value(b"ONAM");
pub const DNAM: RecordType = RecordType::from_value(b"DNAM");
pub const EDID: RecordType = RecordType::from_value(b"EDID");
pub const DATA: RecordType = RecordType::from_value(b"DATA");
pub const XEZN: RecordType = RecordType::from_value(b"XEZN");
pub const OBND: RecordType = RecordType::from_value(b"OBND");
pub const TX00: RecordType = RecordType::from_value(b"TX00");
pub const TX01: RecordType = RecordType::from_value(b"TX01");
pub const TX02: RecordType = RecordType::from_value(b"TX02");
pub const TX03: RecordType = RecordType::from_value(b"TX03");
pub const TX04: RecordType = RecordType::from_value(b"TX04");
pub const TX05: RecordType = RecordType::from_value(b"TX05");
pub const DODT: RecordType = RecordType::from_value(b"DODT");
pub const ICON: RecordType = RecordType::from_value(b"ICON");
pub const MICO: RecordType = RecordType::from_value(b"MICO");
pub const FULL: RecordType = RecordType::from_value(b"FULL");
pub const DESC: RecordType = RecordType::from_value(b"DESC");
pub const ATTR: RecordType = RecordType::from_value(b"ATTR");
pub const XNAM: RecordType = RecordType::from_value(b"XNAM");
pub const RNAM: RecordType = RecordType::from_value(b"RNAM");
pub const MNAM: RecordType = RecordType::from_value(b"MNAM");
pub const FNAM: RecordType = RecordType::from_value(b"FNAM");
pub const INAM: RecordType = RecordType::from_value(b"INAM");
pub const WMI1: RecordType = RecordType::from_value(b"WMI1");
pub const MODL: RecordType = RecordType::from_value(b"MODL");
pub const MODB: RecordType = RecordType::from_value(b"MODB");
pub const MODT: RecordType = RecordType::from_value(b"MODT");
pub const MODS: RecordType = RecordType::from_value(b"MODS");
pub const MODD: RecordType = RecordType::from_value(b"MODD");
pub const MOD2: RecordType = RecordType::from_value(b"MOD2");
pub const MO2T: RecordType = RecordType::from_value(b"MO2T");
pub const MO2S: RecordType = RecordType::from_value(b"MO2S");
pub const MOD3: RecordType = RecordType::from_value(b"MOD3");
pub const MO3T: RecordType = RecordType::from_value(b"MO3T");
pub const MO3S: RecordType = RecordType::from_value(b"MO3S");
pub const MOSD: RecordType = RecordType::from_value(b"MOSD");
pub const MOD4: RecordType = RecordType::from_value(b"MOD4");
pub const MO4T: RecordType = RecordType::from_value(b"MO4T");
pub const MO4S: RecordType = RecordType::from_value(b"MO4S");
pub const HNAM: RecordType = RecordType::from_value(b"HNAM");
pub const INDX: RecordType = RecordType::from_value(b"INDX");
pub const YNAM: RecordType = RecordType::from_value(b"YNAM");
pub const NAM2: RecordType = RecordType::from_value(b"NAM2");
pub const VTCK: RecordType = RecordType::from_value(b"VTCK");
pub const PNAM: RecordType = RecordType::from_value(b"PNAM");
pub const UNAM: RecordType = RecordType::from_value(b"UNAM");
pub const NAM0: RecordType = RecordType::from_value(b"NAM0");
pub const NAM1: RecordType = RecordType::from_value(b"NAM1");
pub const ENAM: RecordType = RecordType::from_value(b"ENAM");
pub const FGGS: RecordType = RecordType::from_value(b"FGGS");
pub const FGGA: RecordType = RecordType::from_value(b"FGGA");
pub const FGTS: RecordType = RecordType::from_value(b"FGTS");
pub const SNDD: RecordType = RecordType::from_value(b"SNDD");
pub const SNDX: RecordType = RecordType::from_value(b"SNDX");
pub const ANAM: RecordType = RecordType::from_value(b"ANAM");
pub const GNAM: RecordType = RecordType::from_value(b"GNAM");
pub const WNAM: RecordType = RecordType::from_value(b"WNAM");
pub const RDAT: RecordType = RecordType::from_value(b"RDAT");

use std::time::Duration;

#[derive(Debug)]
pub struct _Material {
    pub name: &'static str,
    pub temp: u32,
    pub time: Duration,
}

impl _Material {
    pub const PLA: _Material = _Material {
        name: "PLA",
        temp: 45,
        time: Duration::from_secs(60 * 60 * 6),
    };

    pub const PVB: _Material = _Material {
        name: "PVB",
        temp: 45,
        time: Duration::from_secs(60 * 60 * 8),
    };

    pub const PETG: _Material = _Material {
        name: "PETG",
        temp: 55,
        time: Duration::from_secs(60 * 60 * 6),
    };

    pub const ASA: _Material = _Material {
        name: "ASA",
        temp: 80,
        time: Duration::from_secs(60 * 60 * 4),
    };

    pub const TPU: _Material = _Material {
        name: "TPU",
        temp: 60,
        time: Duration::from_secs(60 * 60 * 4),
    };

    pub const NONE: _Material = _Material {
        name: "IDLE",
        temp: 0,
        time: Duration::from_secs(0),
    };

    pub const DEMO: _Material = _Material {
        name: "DEMO",
        temp: 45,
        time: Duration::from_secs(60 * 5),
    };
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    None,
    Demo,
    Pla,
    Pvb,
    Petg,
    Asa,
    Tpu,
}

impl Material {
    pub fn get(&self) -> _Material {
        match self {
            Self::None => _Material::NONE,
            Self::Demo => _Material::DEMO,
            Self::Pla => _Material::PLA,
            Self::Pvb => _Material::PVB,
            Self::Petg => _Material::PETG,
            Self::Asa => _Material::ASA,
            Self::Tpu => _Material::TPU,
        }
    }
    pub fn next(self) -> Self {
        match self {
            Self::None => Self::Demo,
            Self::Demo => Self::Pla,
            Self::Pla => Self::Pvb,
            Self::Pvb => Self::Petg,
            Self::Petg => Self::Asa,
            Self::Asa => Self::Tpu,
            Self::Tpu => Self::None,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Self::None => Self::Tpu,
            Self::Demo => Self::None,
            Self::Pla => Self::Demo,
            Self::Pvb => Self::Pla,
            Self::Petg => Self::Pvb,
            Self::Asa => Self::Petg,
            Self::Tpu => Self::Asa,
        }
    }
}

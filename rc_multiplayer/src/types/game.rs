use oj_rc_core::types::RawType;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Team(u8);

impl RawType for Team {
    type Raw = u8;

    #[inline]
    fn as_raw(&self) -> Self::Raw {
        self.0
    }

    #[inline]
    fn as_raw_ref(&self) -> &'_ Self::Raw {
        &self.0
    }

    #[inline]
    fn from_raw(value: Self::Raw) -> Self {
        Self(value)
    }
}

impl Team {
    #[inline]
    pub fn from_id(team_id: oj_rc_core::types::TeamId) -> Self {
        Self(team_id.as_raw() as u8)
    }

    #[inline]
    pub fn into_id(self) -> oj_rc_core::types::TeamId {
        oj_rc_core::types::TeamId::from_raw(self.0 as i32)
    }

    #[inline]
    pub fn maybe(self) -> MaybeTeam {
        MaybeTeam::from_raw(self.0 as i8)
    }

    #[inline]
    pub fn base(self) -> Base {
        Base::from_raw(self.0)
    }
}

impl core::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Team({})", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct MaybeTeam(i8);

impl MaybeTeam {
    pub const fn unset() -> Self {
        Self(-1)
    }

    #[inline]
    pub fn from_team(team: Team) -> Self {
        Self(team.as_raw() as i8)
    }

    /*#[inline]
    pub fn from_id(team_id: oj_rc_core::types::TeamId) -> Self {
        Self(team_id.as_raw() as i8)
    }

    #[inline]
    pub fn into_id(self) -> oj_rc_core::types::TeamId {
        oj_rc_core::types::TeamId::from_raw(self.0 as i32)
    }*/

    #[inline]
    pub fn as_team(self) -> Option<Team> {
        if self.is_team() {
            Some(Team::from_raw(self.0 as u8))
        } else {
            None
        }
    }

    #[inline]
    pub fn is_team(&self) -> bool {
        self.0 >= 0
    }
}

impl RawType for MaybeTeam {
    type Raw = i8;

    #[inline]
    fn as_raw(&self) -> Self::Raw {
        self.0
    }

    #[inline]
    fn as_raw_ref(&self) -> &'_ Self::Raw {
        &self.0
    }

    #[inline]
    fn from_raw(value: Self::Raw) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for MaybeTeam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaybeTeam({})", self.0)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Base(u8);

impl RawType for Base {
    type Raw = u8;

    #[inline]
    fn as_raw(&self) -> Self::Raw {
        self.0
    }

    #[inline]
    fn as_raw_ref(&self) -> &'_ Self::Raw {
        &self.0
    }

    #[inline]
    fn from_raw(value: Self::Raw) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Base({})", self.0)
    }
}

impl Base {
    #[inline]
    pub fn team(self) -> Team {
        Team::from_raw(self.0)
    }

    #[inline]
    pub fn from_team(team: Team) -> Self {
        Self(team.as_raw())
    }

    #[inline]
    pub fn from_team_id(team: oj_rc_core::types::TeamId) -> Self {
        Self(team.as_raw() as u8)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct CapturePoint(u8);

impl RawType for CapturePoint {
    type Raw = u8;

    #[inline]
    fn as_raw(&self) -> Self::Raw {
        self.0
    }

    #[inline]
    fn as_raw_ref(&self) -> &'_ Self::Raw {
        &self.0
    }

    #[inline]
    fn from_raw(value: Self::Raw) -> Self {
        Self(value)
    }
}

impl core::fmt::Display for CapturePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CapturePoint({})", self.0)
    }
}

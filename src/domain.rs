use gpui::RenderImage;
use uuid::Uuid;
use std::{hash::Hash, sync::Arc};

pub mod prefix {
    pub const TRACK: &str = "00";
    pub const ALBUM: &str = "02";
    pub const ARTIST: &str = "03";
    pub const SEARCH: &str = "04";
    pub const PLAYLIST: &str = "05";
}

// pub trait Entity {
//     fn id(&self) -> String;
// }

#[derive(Clone)]
pub enum KallaxEntity {
    Track(Arc<Track>),
    Album(Arc<Album>),
    Artist(Arc<Artist>),
    Search(Arc<SearchShelf>),
    Playlist(Arc<PlaylistShelf>),
}
impl KallaxEntity {
    pub fn name(&self) -> &str {
        match self {
            KallaxEntity::Track(e) => &e.title,
            KallaxEntity::Album(e) => &e.title,
            KallaxEntity::Artist(e) => &e.name,
            KallaxEntity::Search(e) => &e.name,
            KallaxEntity::Playlist(e) => &e.name,
        }
    }

    pub fn id(&self) -> String {
        match self {
            KallaxEntity::Track(e) => e.id(),
            KallaxEntity::Album(e) => e.id(),
            KallaxEntity::Artist(e) => e.id(),
            KallaxEntity::Search(e) => e.id(),
            KallaxEntity::Playlist(e) => e.id(),
        }
    }
}

pub enum Field {
    Type,
    Id,
    NameOrTitle,
    // Release,
    // Label,
    // ...
}

pub enum LogicalOperator {
    And,
    Or,
    // Not,
}

// pub enum MatchOperator {
//     Is,
//     Contains,
// }

pub enum Expression {
    Logical(LogicalExpression),
    Match(MatchExpression),
}
impl Expression {
    fn matches(&self, entity: &KallaxEntity) -> bool {
        match self {
            Expression::Logical(expr) => expr.matches(entity),
            Expression::Match(expr) => expr.matches(entity),
        }
    }
}

pub struct LogicalExpression {
    pub operator: LogicalOperator,
    pub expressions: Vec<Expression>,
}
impl LogicalExpression {
    fn matches(&self, entity: &KallaxEntity) -> bool {
        match self.operator {
            LogicalOperator::And => self.expressions.iter().all(|expr| expr.matches(entity)),
            LogicalOperator::Or => self.expressions.iter().any(|expr| expr.matches(entity)),
        }
    }
}

pub struct MatchExpression {
    pub field: Field,
    // pub operator: MatchOperator,
    pub value: String,
}
impl MatchExpression {
    fn matches(&self, entity: &KallaxEntity) -> bool {
        match self.field {
            Field::Type => match entity {
                KallaxEntity::Track(_) => self.value == prefix::TRACK,
                KallaxEntity::Album(_) => self.value == prefix::ALBUM,
                KallaxEntity::Artist(_) => self.value == prefix::ARTIST,
                KallaxEntity::Search(_) => self.value == prefix::SEARCH,
                KallaxEntity::Playlist(_) => self.value == prefix::PLAYLIST,
            },
            Field::Id => entity.id() == self.value,
            Field::NameOrTitle => entity.name().contains(&self.value),
        }
    }
}

pub struct SearchShelf {
    pub id: String,
    pub name: String,
    pub expression: Expression,
}
impl SearchShelf {
    pub fn new(name: String, expression: Expression) -> SearchShelf {
        let id = Uuid::new_v4().to_string();
        SearchShelf {
            id,
            name,
            expression,
        }
    }

    pub fn matches(&self, entity: &KallaxEntity) -> bool {
        self.expression.matches(entity)
    }
}
impl SearchShelf {
    pub fn id(&self) -> String {
        format!(
            "{}_{}",
            prefix::SEARCH,
            self.id,
        )
    }
}

pub struct PlaylistShelf {
    pub id: String,
    pub name: String,
    pub entity_ids: Vec<String>,
}
impl PlaylistShelf {
    fn new(name: String) -> PlaylistShelf {
        let id = Uuid::new_v4().to_string();
        PlaylistShelf {
            id,
            name,
            entity_ids: Vec::new(),
        }
    }
}
impl PlaylistShelf {
    pub fn id(&self) -> String {
        format!(
            "{}_{}",
            prefix::PLAYLIST,
            self.id,
        )
    }
}

pub struct Track {
    pub path: String,
    pub title: String,
    pub album_id: String,
    pub artist_id: String,
    pub duration: u32,
    pub track_number: Option<u32>,
    pub disc_number: Option<u32>,
}
impl Track {
    pub fn id(&self) -> String {
        format!(
            "{}_{}_{}_{}",
            prefix::TRACK,
            self.title,
            self.album_id,
            self.artist_id,
        )
    }
}

pub struct Album {
    pub title: String,
    pub sort_title: Option<String>,
    pub artist_id: String,
    pub duration: u32,
    pub artwork: Option<Arc<RenderImage>>,
}
impl Album {
    pub fn id(&self) -> String {
        format!(
            "{}_{}_{}",
            prefix::ALBUM,
            self.sort_title.as_ref().unwrap_or(&self.title),
            self.artist_id,
        )
    }
}
impl Hash for Album {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
impl PartialEq for Album {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
impl Eq for Album {}

pub struct Artist {
    pub name: String,
    pub sort_name: Option<String>,
}
impl Artist {
    pub fn id(&self) -> String {
        format!(
            "{}_{}",
            prefix::ARTIST,
            self.sort_name.as_ref().unwrap_or(&self.name),
        )
    }
}
impl Hash for Artist {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
impl PartialEq for Artist {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
impl Eq for Artist {}

use turbosql::{select, Turbosql};

use crate::{entity_type, Album, Artist, Expression, Field, LogicalExpression, LogicalOperator, MatchExpression, PlaylistShelf, SearchShelf, Track};

#[derive(Clone, Default, Turbosql)]
pub struct DbTrack {
    rowid: Option<i64>,
    pub path: String,
    pub title: String,
    pub album_id: String,
    pub artist_id: String,
    pub duration: u32,
    pub track_number: Option<u32>,
    pub disc_number: Option<u32>,
}
impl DbTrack {
    fn to_domain(self) -> Track {
        // let artwork = self.artwork.and_then(|artwork| {
        //     let cursor: Cursor<Vec<u8>> = Cursor::new(artwork);
        //     let decoder = JpegDecoder::new(cursor).ok()?;
        //     let mut image = DynamicImage::from_decoder(decoder).ok()?.into_rgba8();
        //     for pixel in image.chunks_exact_mut(4) {
        //         let (blue, green, red, alpha) = (pixel[0], pixel[1], pixel[2], pixel[3]);
        //         pixel[0] = red;
        //         pixel[1] = green; // TODO don't need to swap green or alpha here?
        //         pixel[2] = blue;
        //         pixel[3] = alpha;
        //     }
        //     Some(Arc::new(RenderImage::new(vec![Frame::new(image.into())])))
        // });

        Track {
            path: self.path,
            title: self.title,
            album_id: self.album_id,
            artist_id: self.artist_id,
            duration: self.duration,
            track_number: self.track_number,
            disc_number: self.disc_number,
        }
    }

    fn from_domain(track: &Track) -> DbTrack {
        DbTrack {
            rowid: None,
            path: track.path.clone(),
            title: track.title.clone(),
            album_id: track.album_id.clone(),
            artist_id: track.artist_id.clone(),
            duration: track.duration,
            track_number: track.track_number,
            disc_number: track.disc_number,
        }
    }
}
impl AsRef<DbTrack> for DbTrack {
    fn as_ref(&self) -> &DbTrack {
        self
    }
}

#[derive(Clone, Default, Turbosql)]
pub struct DbAlbum {
    rowid: Option<i64>,
    pub title: String,
    pub sort_title: Option<String>,
    pub artist_id: String,
    pub artwork: Option<Vec<u8>>,
}
impl DbAlbum {
    fn to_domain(self) -> Album {
        Album {
            title: self.title,
            sort_title: self.sort_title,
            artist_id: self.artist_id,
            duration: 0,
            artwork: None,
        }
    }

    fn from_domain(album: &Album) -> DbAlbum {
        DbAlbum {
            rowid: None,
            title: album.title.clone(),
            sort_title: album.sort_title.clone(),
            artist_id: album.artist_id.clone(),
            artwork: None,
        }
    }
}
impl AsRef<DbAlbum> for DbAlbum {
    fn as_ref(&self) -> &DbAlbum {
        self
    }
}

#[derive(Clone, Default, Turbosql)]
pub struct DbArtist {
    rowid: Option<i64>,
    pub name: String,
    pub sort_name: Option<String>,
}
impl DbArtist {
    fn to_domain(self) -> Artist {
        Artist {
            name: self.name,
            sort_name: self.sort_name,
        }
    }

    fn from_domain(artist: &Artist) -> DbArtist {
        DbArtist {
            rowid: None,
            name: artist.name.clone(),
            sort_name: artist.sort_name.clone(),
        }
    }
}
impl AsRef<DbArtist> for DbArtist {
    fn as_ref(&self) -> &DbArtist {
        self
    }
}

#[derive(Clone, Default, Turbosql)]
struct DbSearch {
    rowid: Option<i64>,
    pub id: String,
    pub name: String,
    pub expression: String,
}
impl DbSearch {
    fn to_domain(self) -> SearchShelf {
        SearchShelf {
            id: self.id,
            name: self.name,
            expression: Expression::Logical(LogicalExpression { operator: LogicalOperator::And, expressions: Vec::new() }),
        }
    }

    fn from_domain(search: &SearchShelf) -> DbSearch {
        DbSearch {
            rowid: None,
            id: search.id.clone(),
            name: search.name.clone(),
            expression: "TODO".to_string(),
        }
    }
}
impl AsRef<DbSearch> for DbSearch {
    fn as_ref(&self) -> &DbSearch {
        self
    }
}

#[derive(Clone, Default, Turbosql)]
struct DbPlaylist {
    rowid: Option<i64>,
    pub id: String,
    pub name: String,
    pub entity_ids: Vec<String>,
}
impl DbPlaylist {
    fn to_domain(self) -> PlaylistShelf {
        PlaylistShelf {
            id: self.id,
            name: self.name,
            entity_ids: self.entity_ids,
        }
    }

    fn from_domain(playlist: &PlaylistShelf) -> DbPlaylist {
        DbPlaylist {
            rowid: None,
            id: playlist.id.clone(),
            name: playlist.name.clone(),
            entity_ids: playlist.entity_ids.clone(),
        }
    }
}
impl AsRef<DbPlaylist> for DbPlaylist {
    fn as_ref(&self) -> &DbPlaylist {
        self
    }
}

pub fn load() -> (Vec<Track>, Vec<Album>, Vec<Artist>, Vec<SearchShelf>, Vec<PlaylistShelf>) {
    let tracks = select!(Vec<DbTrack>).unwrap();
    let tracks = tracks.into_iter().map(|track| track.to_domain()).collect();

    let albums = select!(Vec<DbAlbum>).unwrap();
    let albums = albums.into_iter().map(|album| album.to_domain()).collect();

    let artists = select!(Vec<DbArtist>).unwrap();
    let artists = artists.into_iter().map(|artist| artist.to_domain()).collect();

    // let searches = select!(Vec<DbSearch>).unwrap();
    // let searches = searches.into_iter().map(|search| search.to_domain()).collect();

    let searches = vec![SearchShelf::new(
        "all tracks".to_string(),
        Expression::Match(MatchExpression {
            field: Field::Type,
            // operator: MatchOperator::Is,
            value: entity_type::TRACK.to_string(),
        }),
    )];

    let playlists = select!(Vec<DbPlaylist>).unwrap();
    let playlists = playlists.into_iter().map(|playlist| playlist.to_domain()).collect();

    (tracks, albums, artists, searches, playlists)
}

pub fn save_tracks(tracks: &[Track]) {
    let tracks: Vec<DbTrack> = tracks.iter().map(DbTrack::from_domain).collect();
    DbTrack::insert_batch(&tracks).ok();
}

pub fn save_albums(albums: &[Album]) {
    let albums: Vec<DbAlbum> = albums.iter().map(DbAlbum::from_domain).collect();
    DbAlbum::insert_batch(&albums).ok();
}

pub fn save_artists(artists: &[Artist]) {
    let artists: Vec<DbArtist> = artists.iter().map(DbArtist::from_domain).collect();
    DbArtist::insert_batch(&artists).ok();
}

use crate::config::{BindingForEvent, ColorTermusic};
use crate::invidious::{Instance, YoutubeVideo};
use crate::podcast::{EpData, PodcastFeed, PodcastNoId};
use crate::songtag::SongTag;
use anyhow::{anyhow, Result};
use image::DynamicImage;
use player::{
    daemon_update::r#Type as GrpcUpdateType, DaemonUpdate as GrpcDaemonUpdate,
    DaemonUpdateChangedTrack as GrpcDaemonUpdateChangedTrack,
};

pub mod player {
    tonic::include_proto!("player");
}

#[derive(Clone, PartialEq, Eq)]
pub struct DaemonUpdateChangedTrack {
    pub new_track_index: u32,
}

#[derive(Clone, PartialEq, Eq)]
pub enum DaemonUpdate {
    ChangedTrack(DaemonUpdateChangedTrack),
}

impl From<DaemonUpdate> for GrpcDaemonUpdate {
    fn from(other: DaemonUpdate) -> Self {
        match other {
            DaemonUpdate::ChangedTrack(data) => GrpcDaemonUpdate {
                r#type: Some(GrpcUpdateType::ChangedTrack(GrpcDaemonUpdateChangedTrack {
                    new_track_index: data.new_track_index,
                })),
            },
        }
    }
}

impl TryFrom<GrpcDaemonUpdate> for DaemonUpdate {
    type Error = anyhow::Error;

    fn try_from(other: GrpcDaemonUpdate) -> Result<DaemonUpdate, Self::Error> {
        match other.r#type {
            Some(GrpcUpdateType::ChangedTrack(data)) => {
                Ok(DaemonUpdate::ChangedTrack(DaemonUpdateChangedTrack {
                    new_track_index: data.new_track_index,
                }))
            }
            _ => Err(anyhow::Error::msg(
                "Could not convert proto DaemonUpdate to rust DaemonUpdate",
            )),
        }
    }
}

/// Messages sent from different actors in the TUI binary and processed by the main TUI thread.
#[derive(Clone, PartialEq, Eq)]
pub enum Msg {
    // AppClose,
    ConfigEditor(ConfigEditorMsg),
    DataBase(DBMsg),
    DeleteConfirmCloseCancel,
    DeleteConfirmCloseOk,
    DeleteConfirmShow,
    Download(DLMsg),
    ErrorPopupClose,
    GeneralSearch(GSMsg),
    HelpPopupShow,
    HelpPopupClose,
    LayoutTreeView,
    LayoutDataBase,
    LayoutPodCast,
    Library(LIMsg),
    LyricMessage(LyricMsg),
    LyricCycle,
    LyricAdjustDelay(i64),
    PlayerToggleGapless,
    PlayerTogglePause,
    PlayerVolumeUp,
    PlayerVolumeDown,
    PlayerSpeedUp,
    PlayerSpeedDown,
    PlayerSeekForward,
    PlayerSeekBackward,
    Playlist(PLMsg),
    Podcast(PCMsg),
    QuitPopupCloseCancel,
    QuitPopupCloseOk,
    QuitPopupShow,
    SavePlaylistPopupShow,
    SavePlaylistPopupCloseCancel,
    SavePlaylistPopupUpdate(String),
    SavePlaylistPopupCloseOk(String),
    SavePlaylistConfirmCloseCancel,
    SavePlaylistConfirmCloseOk(String),
    TagEditor(TEMsg),
    UpdatePhoto,
    YoutubeSearch(YSMsg),
    Xywh(XYWHMsg),
    None,
}

#[derive(Clone, PartialEq, Eq)]
pub enum XYWHMsg {
    Hide,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    ZoomIn,
    ZoomOut,
}

#[derive(Clone, PartialEq, Eq)]
pub enum DLMsg {
    DownloadRunning(String, String), // indicates progress
    DownloadSuccess(String),
    DownloadCompleted(String, Option<String>),
    DownloadErrDownload(String, String, String),
    DownloadErrEmbedData(String, String),
    MessageShow((String, String)),
    MessageHide((String, String)),
    YoutubeSearchSuccess(YoutubeOptions),
    YoutubeSearchFail(String),
    FetchPhotoSuccess(ImageWrapper),
    FetchPhotoErr(String),
}

#[derive(Clone, PartialEq, Eq)]
pub enum LyricMsg {
    LyricTextAreaBlurUp,
    LyricTextAreaBlurDown,
}

#[derive(Clone, PartialEq, Eq)]
pub enum ConfigEditorMsg {
    PodcastDirBlurDown,
    PodcastDirBlurUp,
    PodcastSimulDownloadBlurDown,
    PodcastSimulDownloadBlurUp,
    PodcastMaxRetriesBlurDown,
    PodcastMaxRetriesBlurUp,
    AlbumPhotoAlignBlurDown,
    AlbumPhotoAlignBlurUp,
    ChangeLayout,
    CloseCancel,
    CloseOk,
    ColorChanged(IdConfigEditor, ColorTermusic),
    SymbolChanged(IdConfigEditor, String),
    ConfigChanged,
    ConfigSaveOk,
    ConfigSaveCancel,
    ExitConfirmationBlurDown,
    ExitConfirmationBlurUp,
    Open,
    KeyFocus(KFMsg),
    MusicDirBlurDown,
    MusicDirBlurUp,
    PlaylistDisplaySymbolBlurDown,
    PlaylistDisplaySymbolBlurUp,
    PlaylistRandomTrackBlurDown,
    PlaylistRandomTrackBlurUp,
    PlaylistRandomAlbumBlurDown,
    PlaylistRandomAlbumBlurUp,
    LibraryForegroundBlurDown,
    LibraryForegroundBlurUp,
    LibraryBackgroundBlurDown,
    LibraryBackgroundBlurUp,
    LibraryBorderBlurDown,
    LibraryBorderBlurUp,
    LibraryHighlightBlurDown,
    LibraryHighlightBlurUp,
    LibraryHighlightSymbolBlurDown,
    LibraryHighlightSymbolBlurUp,
    PlaylistForegroundBlurDown,
    PlaylistForegroundBlurUp,
    PlaylistBackgroundBlurDown,
    PlaylistBackgroundBlurUp,
    PlaylistBorderBlurDown,
    PlaylistBorderBlurUp,
    PlaylistHighlightBlurDown,
    PlaylistHighlightBlurUp,
    PlaylistHighlightSymbolBlurDown,
    PlaylistHighlightSymbolBlurUp,
    ProgressForegroundBlurDown,
    ProgressForegroundBlurUp,
    ProgressBackgroundBlurDown,
    ProgressBackgroundBlurUp,
    ProgressBorderBlurDown,
    ProgressBorderBlurUp,
    LyricForegroundBlurDown,
    LyricForegroundBlurUp,
    LyricBackgroundBlurDown,
    LyricBackgroundBlurUp,
    LyricBorderBlurDown,
    LyricBorderBlurUp,
    ThemeSelectBlurDown,
    ThemeSelectBlurUp,
    ThemeSelectLoad(usize),
    KeyChange(IdKey, BindingForEvent),
    SaveLastPositionBlurDown,
    SaveLastPosotionBlurUp,
    SeekStepBlurDown,
    SeekStepBlurUp,
    KillDaemonBlurDown,
    KillDaemonBlurUp,
    PlayerUseMprisBlurDown,
    PlayerUseMprisBlurUp,
    PlayerUseDiscordBlurDown,
    PlayerUseDiscordBlurUp,
    PlayerPortBlurDown,
    PlayerPortBlurUp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KFMsg {
    DatabaseAddAllBlurDown,
    DatabaseAddAllBlurUp,
    GlobalConfigBlurDown,
    GlobalConfigBlurUp,
    GlobalDownBlurDown,
    GlobalDownBlurUp,
    GlobalGotoBottomBlurDown,
    GlobalGotoBottomBlurUp,
    GlobalGotoTopBlurDown,
    GlobalGotoTopBlurUp,
    GlobalHelpBlurDown,
    GlobalHelpBlurUp,
    GlobalLayoutTreeviewBlurDown,
    GlobalLayoutTreeviewBlurUp,
    GlobalLayoutDatabaseBlurDown,
    GlobalLayoutDatabaseBlurUp,
    GlobalLeftBlurDown,
    GlobalLeftBlurUp,
    GlobalLyricAdjustForwardBlurDown,
    GlobalLyricAdjustForwardBlurUp,
    GlobalLyricAdjustBackwardBlurDown,
    GlobalLyricAdjustBackwardBlurUp,
    GlobalLyricCycleBlurDown,
    GlobalLyricCycleBlurUp,
    GlobalPlayerNextBlurDown,
    GlobalPlayerNextBlurUp,
    GlobalPlayerPreviousBlurDown,
    GlobalPlayerPreviousBlurUp,
    GlobalPlayerSeekForwardBlurDown,
    GlobalPlayerSeekForwardBlurUp,
    GlobalPlayerSeekBackwardBlurDown,
    GlobalPlayerSeekBackwardBlurUp,
    GlobalPlayerSpeedUpBlurDown,
    GlobalPlayerSpeedUpBlurUp,
    GlobalPlayerSpeedDownBlurDown,
    GlobalPlayerSpeedDownBlurUp,
    GlobalPlayerToggleGaplessBlurDown,
    GlobalPlayerToggleGaplessBlurUp,
    GlobalPlayerTogglePauseBlurDown,
    GlobalPlayerTogglePauseBlurUp,
    GlobalQuitBlurDown,
    GlobalQuitBlurUp,
    GlobalRightBlurDown,
    GlobalRightBlurUp,
    GlobalUpBlurDown,
    GlobalUpBlurUp,
    GlobalVolumeDownBlurDown,
    GlobalVolumeDownBlurUp,
    GlobalVolumeUpBlurDown,
    GlobalVolumeUpBlurUp,
    GlobalSavePlaylistBlurDown,
    GlobalSavePlaylistBlurUp,
    LibraryDeleteBlurDown,
    LibraryDeleteBlurUp,
    LibraryLoadDirBlurDown,
    LibraryLoadDirBlurUp,
    LibraryPasteBlurDown,
    LibraryPasteBlurUp,
    LibrarySearchBlurDown,
    LibrarySearchBlurUp,
    LibrarySearchYoutubeBlurDown,
    LibrarySearchYoutubeBlurUp,
    LibraryTagEditorBlurDown,
    LibraryTagEditorBlurUp,
    LibraryYankBlurDown,
    LibraryYankBlurUp,
    PlaylistDeleteBlurDown,
    PlaylistDeleteBlurUp,
    PlaylistDeleteAllBlurDown,
    PlaylistDeleteAllBlurUp,
    PlaylistShuffleBlurDown,
    PlaylistShuffleBlurUp,
    PlaylistModeCycleBlurDown,
    PlaylistModeCycleBlurUp,
    PlaylistPlaySelectedBlurDown,
    PlaylistPlaySelectedBlurUp,
    PlaylistSearchBlurDown,
    PlaylistSearchBlurUp,
    PlaylistSwapDownBlurDown,
    PlaylistSwapDownBlurUp,
    PlaylistSwapUpBlurDown,
    PlaylistSwapUpBlurUp,
    PlaylistLqueueBlurDown,
    PlaylistLqueueBlurUp,
    PlaylistTqueueBlurDown,
    PlaylistTqueueBlurUp,
    LibrarySwitchRootBlurDown,
    LibrarySwitchRootBlurUp,
    LibraryAddRootBlurDown,
    LibraryAddRootBlurUp,
    LibraryRemoveRootBlurDown,
    LibraryRemoveRootBlurUp,
    GlobalLayoutPodcastBlurDown,
    GlobalLayoutPodcastBlurUp,
    GlobalXywhMoveLeftBlurDown,
    GlobalXywhMoveLeftBlurUp,
    GlobalXywhMoveRightBlurDown,
    GlobalXywhMoveRightBlurUp,
    GlobalXywhMoveUpBlurDown,
    GlobalXywhMoveUpBlurUp,
    GlobalXywhMoveDownBlurDown,
    GlobalXywhMoveDownBlurUp,
    GlobalXywhZoomInBlurDown,
    GlobalXywhZoomInBlurUp,
    GlobalXywhZoomOutBlurDown,
    GlobalXywhZoomOutBlurUp,
    GlobalXywhHideBlurDown,
    GlobalXywhHideBlurUp,
    PodcastMarkPlayedBlurDown,
    PodcastMarkPlayedBlurUp,
    PodcastMarkAllPlayedBlurDown,
    PodcastMarkAllPlayedBlurUp,
    PodcastEpDownloadBlurDown,
    PodcastEpDownloadBlurUp,
    PodcastEpDeleteFileBlurDown,
    PodcastEpDeleteFileBlurUp,
    PodcastDeleteFeedBlurDown,
    PodcastDeleteFeedBlurUp,
    PodcastDeleteAllFeedsBlurDown,
    PodcastDeleteAllFeedsBlurUp,
    PodcastSearchAddFeedBlurDown,
    PodcastSearchAddFeedBlurUp,
    PodcastRefreshFeedBlurDown,
    PodcastRefreshFeedBlurUp,
    PodcastRefreshAllFeedsBlurDown,
    PodcastRefreshAllFeedsBlurUp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LIMsg {
    TreeExtendDir(String),
    TreeGoToUpperDir,
    TreeBlur,
    Yank,
    Paste,
    SwitchRoot,
    AddRoot,
    RemoveRoot,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DBMsg {
    AddAllToPlaylist,
    AddPlaylist(usize),
    CriteriaBlurDown,
    CriteriaBlurUp,
    SearchResult(usize),
    SearchResultBlurDown,
    SearchResultBlurUp,
    SearchTrack(usize),
    SearchTracksBlurDown,
    SearchTracksBlurUp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PCMsg {
    PodcastBlurDown,
    PodcastBlurUp,
    EpisodeBlurDown,
    EpisodeBlurUp,
    PodcastAddPopupShow,
    PodcastAddPopupCloseOk(String),
    PodcastAddPopupCloseCancel,
    SyncData((i64, PodcastNoId)),
    NewData(PodcastNoId),
    Error(String, PodcastFeed),
    PodcastSelected(usize),
    DescriptionUpdate,
    EpisodeAdd(usize),
    EpisodeMarkPlayed(usize),
    EpisodeMarkAllPlayed,
    PodcastRefreshOne(usize),
    PodcastRefreshAll,
    FetchPodcastStart(String),
    EpisodeDownload(usize),
    DLStart(EpData),
    DLComplete(EpData),
    DLResponseError(EpData),
    DLFileCreateError(EpData),
    DLFileWriteError(EpData),
    EpisodeDeleteFile(usize),
    FeedDeleteShow,
    FeedDeleteCloseOk,
    FeedDeleteCloseCancel,
    FeedsDeleteShow,
    FeedsDeleteCloseOk,
    FeedsDeleteCloseCancel,
    SearchItunesCloseCancel,
    SearchItunesCloseOk(usize),
    SearchSuccess(Vec<PodcastFeed>),
    SearchError(String),
}

/// This message is sent by the UI due to keypress events related to the playlist
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PLMsg {
    NextSong,
    PrevSong,
    PlaylistTableBlurDown,
    PlaylistTableBlurUp,
    Add(String),
    Delete(usize),
    DeleteAll,
    LoopModeCycle,
    PlaySelected(usize),
    Shuffle,
    SwapDown(usize),
    SwapUp(usize),
    CmusLQueue,
    CmusTQueue,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GSMsg {
    PopupShowDatabase,
    PopupShowLibrary,
    PopupShowPlaylist,
    PopupCloseCancel,
    InputBlur,
    PopupUpdateDatabase(String),
    PopupUpdateLibrary(String),
    PopupUpdatePlaylist(String),
    TableBlur,
    PopupCloseDatabaseAddPlaylist,
    PopupCloseLibraryAddPlaylist,
    PopupCloseOkLibraryLocate,
    PopupClosePlaylistPlaySelected,
    PopupCloseOkPlaylistLocate,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum YSMsg {
    InputPopupShow,
    InputPopupCloseCancel,
    InputPopupCloseOk(String),
    TablePopupNext,
    TablePopupPrevious,
    TablePopupCloseCancel,
    TablePopupCloseOk(usize),
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TEMsg {
    TagEditorRun(String),
    TagEditorClose(Option<String>),
    TECounterDeleteOk,
    TEDownload(usize),
    TEEmbed(usize),
    TEFocus(TFMsg),
    TERename,
    TESearch,
    TESelectLyricOk(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TFMsg {
    CounterDeleteBlurDown,
    CounterDeleteBlurUp,
    InputArtistBlurDown,
    InputArtistBlurUp,
    InputTitleBlurDown,
    InputTitleBlurUp,
    InputAlbumBlurDown,
    InputAlbumBlurUp,
    InputGenreBlurDown,
    InputGenreBlurUp,
    SelectLyricBlurDown,
    SelectLyricBlurUp,
    TableLyricOptionsBlurDown,
    TableLyricOptionsBlurUp,
    TextareaLyricBlurDown,
    TextareaLyricBlurUp,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    ConfigEditor(IdConfigEditor),
    DBListCriteria,
    DBListSearchResult,
    DBListSearchTracks,
    DeleteConfirmRadioPopup,
    DeleteConfirmInputPopup,
    DownloadSpinner,
    Episode,
    ErrorPopup,
    GeneralSearchInput,
    GeneralSearchTable,
    GlobalListener,
    HelpPopup,
    Label,
    Library,
    Lyric,
    MessagePopup,
    Playlist,
    Podcast,
    PodcastAddPopup,
    PodcastSearchTablePopup,
    FeedDeleteConfirmRadioPopup,
    FeedDeleteConfirmInputPopup,
    Progress,
    QuitPopup,
    SavePlaylistPopup,
    SavePlaylistLabel,
    SavePlaylistConfirm,
    TagEditor(IdTagEditor),
    YoutubeSearchInputPopup,
    YoutubeSearchTablePopup,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum IdTagEditor {
    CounterDelete,
    LabelHint,
    InputArtist,
    InputTitle,
    InputAlbum,
    InputGenre,
    SelectLyric,
    TableLyricOptions,
    TextareaLyric,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum IdConfigEditor {
    AlbumPhotoAlign,
    CEThemeSelect,
    ConfigSavePopup,
    ExitConfirmation,
    Footer,
    Header,
    Key(IdKey),
    KillDamon,
    LibraryBackground,
    LibraryBorder,
    LibraryForeground,
    LibraryHighlight,
    LibraryHighlightSymbol,
    LibraryLabel,
    LyricBackground,
    LyricBorder,
    LyricForeground,
    LyricLabel,
    MusicDir,
    PlayerPort,
    PlayerUseDiscord,
    PlayerUseMpris,
    PlaylistBackground,
    PlaylistBorder,
    PlaylistDisplaySymbol,
    PlaylistForeground,
    PlaylistHighlight,
    PlaylistHighlightSymbol,
    PlaylistLabel,
    PlaylistRandomAlbum,
    PlaylistRandomTrack,
    PodcastDir,
    PodcastMaxRetries,
    PodcastSimulDownload,
    ProgressBackground,
    ProgressBorder,
    ProgressForeground,
    ProgressLabel,
    SaveLastPosition,
    SeekStep,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum IdKey {
    DatabaseAddAll,
    GlobalConfig,
    GlobalDown,
    GlobalGotoBottom,
    GlobalGotoTop,
    GlobalHelp,
    GlobalLayoutTreeview,
    GlobalLayoutDatabase,
    GlobalLeft,
    GlobalLyricAdjustForward,
    GlobalLyricAdjustBackward,
    GlobalLyricCycle,
    GlobalPlayerToggleGapless,
    GlobalPlayerTogglePause,
    GlobalPlayerNext,
    GlobalPlayerPrevious,
    GlobalPlayerSeekForward,
    GlobalPlayerSeekBackward,
    GlobalPlayerSpeedUp,
    GlobalPlayerSpeedDown,
    GlobalQuit,
    GlobalRight,
    GlobalUp,
    GlobalVolumeDown,
    GlobalVolumeUp,
    GlobalSavePlaylist,
    LibraryDelete,
    LibraryLoadDir,
    LibraryPaste,
    LibrarySearch,
    LibrarySearchYoutube,
    LibraryTagEditor,
    LibraryYank,
    PlaylistDelete,
    PlaylistDeleteAll,
    PlaylistShuffle,
    PlaylistModeCycle,
    PlaylistPlaySelected,
    PlaylistSearch,
    PlaylistSwapDown,
    PlaylistSwapUp,
    PlaylistLqueue,
    PlaylistTqueue,
    LibrarySwitchRoot,
    LibraryAddRoot,
    LibraryRemoveRoot,
    GlobalLayoutPodcast,
    GlobalXywhMoveLeft,
    GlobalXywhMoveRight,
    GlobalXywhMoveUp,
    GlobalXywhMoveDown,
    GlobalXywhZoomIn,
    GlobalXywhZoomOut,
    GlobalXywhHide,
    PodcastMarkPlayed,
    PodcastMarkAllPlayed,
    PodcastEpDownload,
    PodcastEpDeleteFile,
    PodcastDeleteFeed,
    PodcastDeleteAllFeeds,
    PodcastSearchAddFeed,
    PodcastRefreshFeed,
    PodcastRefreshAllFeeds,
}
pub enum SearchLyricState {
    Finish(Vec<SongTag>),
}

#[derive(Clone, PartialEq)]
pub struct ImageWrapper {
    pub data: DynamicImage,
}
impl Eq for ImageWrapper {}

#[derive(Clone, PartialEq, Eq)]
pub struct YoutubeOptions {
    pub items: Vec<YoutubeVideo>,
    pub page: u32,
    pub invidious_instance: Instance,
}

impl Default for YoutubeOptions {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            page: 1,
            invidious_instance: crate::invidious::Instance::default(),
        }
    }
}

impl YoutubeOptions {
    pub fn get_by_index(&self, index: usize) -> Result<&YoutubeVideo> {
        if let Some(item) = self.items.get(index) {
            return Ok(item);
        }
        Err(anyhow!("index not found"))
    }

    pub fn prev_page(&mut self) -> Result<()> {
        if self.page > 1 {
            self.page -= 1;
            self.items = self.invidious_instance.get_search_query(self.page)?;
        }
        Ok(())
    }

    pub fn next_page(&mut self) -> Result<()> {
        self.page += 1;
        self.items = self.invidious_instance.get_search_query(self.page)?;
        Ok(())
    }

    pub const fn page(&self) -> u32 {
        self.page
    }
}

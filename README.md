# rsteam
[![Version](https://img.shields.io/crates/v/rsteam)](https://crates.io/crates/rsteam)
[![License](https://img.shields.io/github/license/KnoxZZ/rsteam)](https://github.com/KnoxZZ/rsteam/blob/master/LICENSE)
[![Docs](https://img.shields.io/docsrs/rsteam/latest)](https://docs.rs/rsteam)
[![Downloads](https://img.shields.io/crates/d/rsteam)](https://crates.io/crates/rsteam)

Easy to use API wrapper for steam web API.
- Asynchronous

Get started by checking the [examples](examples/)

Currently supported interfaces:
- ISteamUser
    - ResolveVanityURL
    - GetPlayerSummaries
    - GetPlayerBans
    - GetFriendList
    - GetUserGroupList
- ISteamUserStats
    - GetGlobalAchievementPercentagesForApp
    - GetNumberOfCurrentPlayers
    - GetUserStatsForGame
- IPlayerService
    - GetBadges
    - GetCommunityBadgeProcess
    - GetOwnedGames
    - GetRecentlyPlayedGames
    - GetSteamLevel
    - IsPlayingSharedGame
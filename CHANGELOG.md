# v0.5.1 (hotfix) 2023-12-01

## Fixes

- Decoding error when the property `ts` of the object "badges" was not present

---

# v0.5.0 2023-11-30

## Fixes

- Decoding error caused by the TWC badges [[#4](https://github.com/Rinrin0413/tetr-ch-rs/issues/4)]
- Missing property `currentbtbchainpower` in struct `SinglePlayEndCtx` [[#6](https://github.com/Rinrin0413/tetr-ch-rs/issues/6)]
- Some outdated example code (01, 04)
- Some typos in document and `CHANGELOG.md`

## Additions

- Support new type of line clear "Pentas" and "T-Spin Pentas" [[#7](https://github.com/Rinrin0413/tetr-ch-rs/issues/7)]
- A field `group` in struct `Badge` [[#5](https://github.com/Rinrin0413/tetr-ch-rs/issues/5)]

## Changes

- Rename a field `attack` to `attacks` of struct `EndCtxGarbage`

## Improvements

- Improve the document
- Update the library description sentence in  `README.md` and `/src/lib.rs`

## Internal

- Update `.gitignore`

---

# v0.4.0 2023-06-29

## Changes

- License changed from GPL-3.0 to MIT.
- Support TETRA LEAGUE streams and multi player records [[#3](https://github.com/Rinrin0413/tetr-ch-rs/issues/3)]  
This has significantly changed the structure around records.

## Fixes

- Fix some typos.

## Improvements

- Make `ResponseError` a standard error type by [@jlkn](https://github.com/jlkn) in [[#2](https://github.com/Rinrin0413/tetr-ch-rs/pull/2)]

---

# v0.3.5 2023-05-23

## Improvements

- Supported TETR.IO v6.3.4 ([issue #1](https://github.com/Rinrin0413/tetr-ch-rs/issues/1))

## Fixes

- Fixed redundant and raggedly named functions.

---

# v0.3.4 2022-12-18

---

# v0.3.3 2022-12-12

---

# v0.3.2 2022-10-28

## Additions

- Added module [`profile_url`](https://docs.rs/tetr_ch/0.3.2/tetr_ch/model/searched_user/struct.UserInfo.html#method.profile_url) to [`UserInfo`](https://docs.rs/tetr_ch/0.3.2/tetr_ch/model/searched_user/struct.UserInfo.html) and parent structs.
- Added modules [`cached_at`](https://docs.rs/tetr_ch/0.3.2/tetr_ch/model/searched_user/struct.SearchedUserResponse.html#method.cached_at) and [`cached_until`](https://docs.rs/tetr_ch/0.3.2/tetr_ch/model/searched_user/struct.SearchedUserResponse.html#method.cached_until) to [`SearchedUserResponse`](https://docs.rs/tetr_ch/0.3.2/tetr_ch/model/searched_user/struct.SearchedUserResponse.html).

## Improvements

- Improved the documentation.

---

# v0.3.1  2022-10-25

- Fixed forgot to update.

---

# v0.3.0 (support for TETR.IO v6.3.1) 2022-10-25

## Additions

- Added a method [`search_user`](https://docs.rs/tetr_ch/0.3.0/tetr_ch/client/struct.Client.html#method.search_user) to [`Client`](https://docs.rs/tetr_ch/0.3.0/tetr_ch/client/struct.Client.html). You can find TETR.IO users from the Discord account with this method.
- Added about highest achieved rank (`best_rank`) to [`LeagueData`](https://docs.rs/tetr_ch/0.3.0/tetr_ch/model/league/struct.LeagueData.html) and [`LeagueDataMini`](https://docs.rs/tetr_ch/0.3.0/tetr_ch/model/league_leaderboard/struct.LeagueDataMini.html).
- Added connections and distinguishment fields to [`User`](https://docs.rs/tetr_ch/0.3.0/tetr_ch/model/user/struct.User.html).

## Improvements

- Some methods now returns `Option<T>` for ease of use. Especially with rank icons and rank colors.
- Improved performance of some methods.

## Fixes

- Fixed some typos...
- Added some missing methods to the models.

---

# v0.2.1 (hotfix)  2022-09-16

- Fixed the inverse inequality sign.

---

# v0.2.0  2022-09-16

- Added some useful methods.
- Many elements to appropriate names.
- Made the module [`rank_col`](https://docs.rs/tetr_ch/0.2.0/tetr_ch/constants/rank_col/) deprecated.
- Improved performance.
- Improved the documentation.
- Moved [`LeagueData`](https://docs.rs/tetr_ch/0.1.1/tetr_ch/model/user/struct.LeagueData.html) to new module [`league`](https://docs.rs/tetr_ch/0.2.0/tetr_ch/model/league/).
- Prepared a league rank exclusive type at new module [`league`](https://docs.rs/tetr_ch/0.2.0/tetr_ch/model/league/).
- Implemented trait `Default` to implementable types.
- Fixed missing dependencies `tokio` in [ex01](./examples/ex01_get_user_details/).

---

# v0.1.1  2022-08-28

- Added some examples.
- Added some tests.
- Deprecated the [`rank`](https://docs.rs/tetr_ch/0.1.1/tetr_ch/model/latest_news/struct.NewsData.html#structfield._rank) field of [`NewsData`](https://docs.rs/tetr_ch/latest/tetr_ch/model/latest_news/struct.NewsData.html).
- Improved the documentation.
- Fixed some typos.

---

# v0.1.0  2022-08-23

Initial release

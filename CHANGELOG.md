# v0.7.0 2024-12-26

## Breaking Changes

- ✨ New struct [`Response<T>`](https://docs.rs/tetr_ch/0.7.0/tetr_ch/model/response/struct.Response.html) has been introduced to wrap the response data instead of the [`*Response`](https://docs.rs/tetr_ch/0.6.1/tetr_ch/client/index.html?search=model%20Response) structs. [#114]
    - ⚰️ As a result, the [`*Response`](https://docs.rs/tetr_ch/0.6.1/tetr_ch/client/index.html?search=model%20Response) structs were removed [fac6708f84d1c6d500531035c1537fa85ce61857]

## Bug Fixes

- 🐛 The [`record_leaderboard::SearchCriteria::init`](https://docs.rs/tetr_ch/0.7.0/tetr_ch/client/param/record_leaderboard/struct.SearchCriteria.html#method.init) method was incorrectly setting the [`limit`](https://docs.rs/tetr_ch/0.7.0/tetr_ch/client/param/record_leaderboard/struct.SearchCriteria.html#structfield.limit) field to `Some(25)` instead of initializing to `None`. [79b3dd95a8675fada7aeb043226904edee07f9d7]
- 🩹 Three models had not satisfied the trait bound [`AsRef<T>`](https://doc.rust-lang.org/nightly/core/convert/trait.AsRef.html). [c9911b68f99a6ed38b648625e246ffd1aaa0ee43]
    - [`RecordsLeaderboard`](https://docs.rs/tetr_ch/0.7.0/tetr_ch/model/records_leaderboard/struct.RecordsLeaderboard.html) struct
    - [`AllSummaries`](https://docs.rs/tetr_ch/0.7.0/tetr_ch/model/summary/struct.AllSummaries.html) struct
    - [`UserRecords`](https://docs.rs/tetr_ch/0.7.0/tetr_ch/model/user_records/struct.UserRecords.html) struct

## Improvements

- 📚 Fixed some incorrect examples in the documentation. [78d851cbcb2e6e36fedf1dcb8bdfef97ae0d5b61]

## Other Changes

- 🚚 Moved the [`ErrorResponse`](https://docs.rs/tetr_ch/0.7.0/tetr_ch/model/response/struct.ErrorResponse.html) struct to [`crate::model::response`](https://docs.rs/tetr_ch/0.7.0/tetr_ch/model/response/index.html) module. [3c86e6d1299d1d42243eb52b7c8702eec9123099]

## Internal Changes

- ✅ Wrote new unit and integration tests.
- ✅ Removed the unnecessary integration tests.

---

# v0.6.1 2024-12-11

## Features

- ✨ Re-exported the [`Client`](https://docs.rs/tetr_ch/0.6.1/tetr_ch/client/struct.Client.html) struct to the [crate root](https://docs.rs/tetr_ch/0.6.1/tetr_ch/index.html#reexports). [#106]

## Bug Fixes

- 🐛 Fixed deserialization errors when using the [`Client::get_user_league`](https://docs.rs/tetr_ch/0.6.1/tetr_ch/client/struct.Client.html#method.get_user_league) method. [#102 #107]
    - 🛠️ The [`LeagueData`](https://docs.rs/tetr_ch/0.6.1/tetr_ch/model/summary/league/struct.LeagueData.html) struct is now wrapped in new enum [`LeagueDataWrap`](https://docs.rs/tetr_ch/0.6.1/tetr_ch/model/summary/league/enum.LeagueDataWrap.html). [f2e0f232c239f85b1bbe5162f41a938c3057dbfc]
- ⚰️ Removed an unnecessary `dbg` macro call in the [`Client::get_user`](https://docs.rs/tetr_ch/0.6.1/tetr_ch/client/struct.Client.html#method.get_user) method implementation. [#104]

## Other Changes

- 📚 Fixed an incorrect link in the `CHANGELOG.md`. [#103]
- 🛠️ Adjusted the examples. [c63f5137e0596fcb05ba2dc9897a3d6521f75599]

---

# v0.6.0 2024-12-07

## Breaking Changes

- 💥 [TETR.IO BETA 1.2.0](https://tetr.io/about/patchnotes/#chlog_BETA_1_2_0)+ are now supported. [#15]
    - ✨ Added new methods of the [`Client`](https://docs.rs/tetr_ch/0.6.0/tetr_ch/client/struct.Client.html) struct for the new API endpoints.
        - 🔥 Removed the methods and elements for the discontinued endpoints. [#64]
    - ✨ Added support for the new rank X+. [#61]

## Features

- ✨ The [`Client`](https://docs.rs/tetr_ch/0.6.0/tetr_ch/client/struct.Client.html) struct now supports `X-Session-ID` header. [#97]
    - Use [`Client::with_session_id`](https://docs.rs/tetr_ch/0.6.0/tetr_ch/client/struct.Client.html#method.with_session_id).
- ✨ Added two prelude modules [`tetr_ch::prelude`](https://docs.rs/tetr_ch/0.6.0/tetr_ch/prelude/index.html) and [`tetr_ch::model::prelude`](https://docs.rs/tetr_ch/0.6.0/tetr_ch/model/prelude/index.html).
- ✨ Added [`xp_to_level`](https://docs.rs/tetr_ch/0.6.0/tetr_ch/util/fn.xp_to_level.html) function.

## Improvements

- 🛠️ Parameters that are used in the URL are now encoded.
- 📚 Improved the documentation.

## Other Changes

- 🛠️ The enumerators of the [`ResponseError`](https://docs.rs/tetr_ch/0.6.0/tetr_ch/client/error/enum.ResponseError.html) enum now have each error type. [#89]

---

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

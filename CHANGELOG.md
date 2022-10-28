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

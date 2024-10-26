# fenlu

simple and extensible all-purpose media organizer inspired by qimgv

## dependencies

### build dependencies - fenlu

- qt6
- pkg-config
- g++
- [rust](https://www.rust-lang.org/)

### runtime dependencies - fenlu

- qt6

## faq

<details>
  <summary>Q: what does fenlu mean?</summary>
  A: it was a random name that i translated to chinese
  and means "journal entry" (according to google translate atleast)
  so i took it as: this application being the journal, and the media your entries!
</details>

<details>
  <summary>Q: how do i script this?</summary>
  A: any binary, shell script, .py, or .js file
  can be placed in the `scripts/` folder
  and be added to config.whitelisted_scripts
  the protocol for scripts is defined in `src/protocol`
</details>

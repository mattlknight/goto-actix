# goto [![Build Status](https://travis-ci.org/mattlknight/goto-actix.svg?branch=master)](https://travis-ci.org/mattlknight/goto-actix)

A URL Shortener and Redirector Web App
- Cloning this repo requires cloning submodules as well
 - `git clone --recurse-submodules -j8 <repo_url_here>`

[Rust Docs](https://mattlknight.github.io/goto-actix/)

# Dev Machine Setup
- postgres-11
 - `sudo apt-get install curl ca-certificates`
 - `curl https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -`
 - `sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt/ bionic-pgdg main" > /etc/apt/sources.list.d/pgdg.list'`
   - Issues with using cosmic, use bionic here
 - `sudo apt update`
 - `sudo apt install postgresql-11 pgadmin4 libpq-dev`
- rustup
 - `curl https://sh.rustup.rs -sSf | sh`
 - `rustup default nightly`
- diesel-cli `cargo install diesel_cli --no-default-features --features postgres`
- Atom Packages
 - ide-rust
 - language-rust
- Postman
 - Download and unpack tar.gz from `https://dl.pstmn.io/download/latest/linux64`
 - `mkdir ~/bin`
 - `ln -s $HOME/Downloads/Postman/Postman $HOME/bin`
 - Verify successful link `cd ~/bin && ls -alh`
 - Add convenience link `ln -s Postman postman`
 - Add `export PATH="$HOME/bin:$PATH"` to `~/.zshrc`
- Before running server, you need to configure postgres properly
 - On postgres server as regular user `sudo su - postgres`
 - Connect to pg cli `psql`
 - Create a new role(user) `create user goto with password 'gotopassword' createdb;`
- Intialize database with diesel
 - Initial One Time Setup `diesel database setup`
  - Or subsequent runs `diesel database reset`
  - Both automatically run the migrations
- Test API Server
 - `cargo run`
 - `postman`
  - You can skip sign in
- Try a GET request (`rust` keyword is embedded in migration)
 - GET `http://localhost:8080/api/keyword/rust`
  - You should get back
`{
    "keyword": "rust",
    "url": "https://www.rust-lang.org/"
}`
- Try a POST request
 - POST `http://localhost:8080/api/keyword`
 - HEADER `content-type application/json`
 - BODY/raw/json `{ "keyword": "test", "url": "http://test.com/"}`
   - You can use https://jsonlint.com/ to validate your json
  - Successful setup should return a 200OK and the same keyword pair you POST'ed

## API Development
- Use the swagger-editor, before creating/updating rust source code
 - `cd swagger-editor && npm start`

## Design
- `<hostname>` can be goto or go, or whatever you want
- `<hostname>/` Index page, form to input new (keyword, url) pair
- `<hostname>/admin`, admin page to view list of mapped shortcuts
- `<hostname>/error`, error page to indicate invalid keyword passed, link to index page
- `<hostname>/<keyword>`, lookup keyword in DB
  - If keyword exists in DB, temporary(?) redirect to mapped URL
  - If not exists, redirect to `<hostname>/error`

## Bonus
- Track keywords and increment a counter in DB
- Track keyword creation and modification times
- Show top 10(+) keywords as a link on index home page
- Show 10(+) most recently modified(incl. created) keywords as links on index page
- Auto delete tracking activity older than 90 days

## DB Layout
- table: keywords
  - row_id: pkey
  - keyword: varchar[64] indexed
  - url: varchar[512]
  - created_on: tzdate, indexed
  - modified_on: tzdate, indexed
- table: tracking
  - row_id: pkey
  - keyword: foreign key keywords.keyword, indexed
  - accessed_on: tzdate, indexed

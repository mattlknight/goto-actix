# goto
A URL Shortener and Redirector Web App

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

# goto
A URL Shortener and Redirector Web App

## Design
- goto/ Index page, form to input new (keyword, url) pair
- goto/admin, admin page to view list of mapped shortcuts
- goto/error, error page to indicate invalid keyword passed, link to index page
- `goto/<keyword>`, lookup keyword in DB
  - If keyword exists in DB, redirect to mapped URL
  - If not exists, redirect to <hostname>/error

## Bonus
- Track keywords and increment a counter in DB
- Track keyword creation and modification times
- Show top 10(+) keywords as a link on index home page
- Show 10(+) most recently modified(incl. created) keywords as links on index page

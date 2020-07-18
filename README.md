# Smart-Search
A tool that lets you create smart bookmarks and search your frequent webpages efficiently.

## Set Up

### Setting Up with Firefox

To set up the new application to be used as a custom search engine in Firefox, you can follow these steps:

- Install the “Add custom search engine” Firefox Add-on
- Open up the extension
- Fill out the form with the following values:
  - Name: Smart Search (you can use whatever you want here)
  - Search URL: https://protected-bayou-98664.herokuapp.com/search?cmd=%s or http://localhost:8000/search?cmd=%s (For local testing)
- Click “Add custom search engine”
- Check the box “Make this the current search engine”
- Click “Add”

If your app isn’t already running locally, run cargo run.

Open a new tab in Firefox and try some of the commands we added! With that, the app is working locally

### Setting up with Chrome

Setting up with Chrome is a little bit more straightforward because you don’t need any extensions or add-ons. Follow these steps:

- Navigate to chrome://settings/searchEngines
- Click “Add” under “Default Search Engines” and use the following values:
  - Search Engine: Smart-Search
  - Keyword: ss (triggers the search engine, if this search engine is not the default)
  - URL: https://protected-bayou-98664.herokuapp.com/search?cmd=%s or http://localhost:8000/search?cmd=%s (For local testing)
- Under “Other search engines”, find your search engine, select the 3 dots menu and select “Make default”

## Usage
In the address bar of the browser type the search query with supported commands

The following commands are supported by smart-search:

- "ld" -> redirects to linkedin.com
- "ld @username" -> redirects to linkedin.com/in/username
- "ld searchvalue" -> redirects to serch results page of linkedin with given searchvalue
- "gh" -> redirects to github.com
- "gh username" -> redirects to github.com/username
- "gh username/repo" -> redirects to github.com/username/repo
- "cd @username" -> redirects to codeforces/profile/username
- "cd comma_seperated_problem_domains " -> redirects to the codeforces problem search result page with problems of asked domain 
- "tw" -> redirects to twitter.com
- "tw @username" -> redirects to twitter.com/username

Everything else redirects to a google search with your query.

## Local Testing and Setup
- Make sure RUST is installed in your system.
- Clone the repository and run the app.
- Use local url for setting up your browser.

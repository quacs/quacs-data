# QuACS Data
This repository stores the catalog information for QuACS, the Questionably Accurate Course Scheduler.  View the live website at https://quacs.org/ or visit our main repository at https://github.com/quacs/quacs.

## Rerun Scraper
You must have push access to rerun the scraper.

1. Generate a github token with the repo scope. [Learn More](https://docs.github.com/en/github/authenticating-to-github/creating-a-personal-access-token)
2. Run this curl command with your token
```bash
curl -H "Accept: application/vnd.github.everest-preview+json" \
    -H "Authorization: token <your-token-here>" \
    --request POST \
    --data '{"event_type": "rescrape"}' \
    https://api.github.com/repos/quacs/quacs-data/dispatches
```

NOTE: Please do not do this too often or I will have to disable this ability.

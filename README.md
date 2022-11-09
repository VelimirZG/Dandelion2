Dandelion
==================

This repository contains an implementation of the main contract of Dandelion app. 

### Prerequisits
In order to interact with smart contract, first it needs to be deployed. 

1. Clone this repo locally\
       ```git clone https://github.com/VelimirZG/Dandelion.git```
2. Change directory to Dandelion\
       ```cd dandelion```
2. Install dependencies\
       ```yarn install```\
       ```yarn build```
3. Deploy the contract\
   ```near dev-deploy -f --wasmFile out/main.wasm```

Once we have contract deployed, use the Account ID where it was deployed to set an environment variable to make it easy to copy and paste(alternatively you can use your own account ID):

export NEARID=AccountID\
   for example: ```export NEARID=dev-1661857720366-11013681566272```

### Interact with contract

1. **Initialize the contract:**\
   ```near call $NEARID new_default_meta '{"contract_owner_id": "'$NEARID'"}' --accountId $NEARID```
   
2. **Create idea:**\
```near call $NEARID create_idea '{"idea_id":1, "metadata": { "title": "Blockchain game characters", "description": "Building a blockchain character ecosytem", "picture_url": "https://bafybeidl4hjbpdr6u6xvlrizwxbrfcyqurzvcnn5xoilmcqbxfbdwrmp5m.ipfs.dweb.link/", "tags":["economy","games", "ecosystem"],"value_proposition":"blockchain based platform", "owner_id":"'$NEARID'", "team": {"name":"Jonny doe", "cv":"This is some CV"}}}' --accountId $NEARID```

3. **View idea info**\
```near view $NEARID get_idea '{"idea_id": 1}'```

4. **View ideas per creator**\
```near view $NEARID get_ideas_by_owner_id '{"owner_id":"'$NEARID'"}'```

5. **Set phase goals for idea**\
```near call $NEARID update_or_create_goals '{"idea_id":1,"project_phase":2, "amount":1}' --accountId $NEARID```

6. **Invest in idea**\
```near call $NEARID invest_in_idea '{"idea_id":1,"project_phase":4}' --accountId $NEARID --deposit 0.8```

7. **Check how much is invested in idea**\
```near call $NEARID get_total_invested_by_idea '{"idea_id":1}' --accountId $NEARID```

8. **View all investments**\
```near view $NEARID get_all_investments '{"from_index":0,"limit":100}'```

9. **View total number of investments**\
```near view $NEARID total_investments```

10. **View investment goal for idea**\
```near view $NEARID get_goals '{"idea_id":1}'```

11. **View all ideas**\
```near view $NEARID get_all_ideas '{"from_index":0,"limit":100}'```

TODO:
12. **Collect investment**\
```near view $NEARID get_all_ideas```

13. **Return investment if not realized**\
```near view $NEARID get_all_ideas```



### FRONTEND


1. **Switch to frontend POC:**\
   ```cd frontend-poc```

In the project directory, you can run:

### `npm start`

Runs the app in the development mode.\
Open [http://localhost:3000](http://localhost:3000) to view it in your browser.# Dandelion2

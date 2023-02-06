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
```near call $NEARID create_idea '{"idea_id":12, "metadata": { "title": "Blockchain game characters", "description": "Building a blockchain character ecosytem", "picture_url": "https://bafybeidl4hjbpdr6u6xvlrizwxbrfcyqurzvcnn5xoilmcqbxfbdwrmp5m.ipfs.dweb.link/", "tags":["economy","games", "ecosystem"],"value_proposition":"blockchain based platform", "owner_id":"'$NEARID'", "team": {"name":"Jonny doe", "cv":"This is some CV"}}, "amount":1}' --accountId $NEARID```

3. **View idea info**\
```near view $NEARID get_idea '{"idea_id": 1}'```

4. **View ideas per creator**\
```near view $NEARID get_ideas_by_owner_id '{"owner_id":"'$NEARID'"}'```
Need to be replaced by more complex view

5. **Set phase goals for idea**\
```near call $NEARID create_goals '{"idea_id":1,"project_phase":2, "amount":1}' --accountId $NEARID```

6. **Invest in idea**\
```near call $NEARID invest_in_idea '{"idea_id":1,"project_phase":2}' --accountId $NEARID --deposit 0.8```

7. **Check how much is invested in idea**\
```near call $NEARID get_total_invested_by_idea '{"idea_id":1}' --accountId $NEARID```

8. **View all investments**\
```near view $NEARID get_all_investments '{"from_index":0,"limit":100}'```
This one needs to be replaced with more complex view


10. **View investment goal for idea**\
```near view $NEARID get_goals '{"idea_id":1}'```

11. **View all ideas**\
```near view $NEARID get_all_ideas '{"from_index":0,"limit":100}'```

11. **View all ideas for homepage**\
```near view $NEARID get_all_ideas_homepage '{"from_index":0,"limit":20}'```

11. **Get idea for single**\
```near view $NEARID get_idea_for_single '{"idea_id":16}'```

**DASHBOARD:**\
11. **Get sum of received investments for ideas of one owner - check does it need pagination**\
```near view $NEARID get_sum_of_amount_for_owner '{"owner_id":"'$NEARID'"}'``` 

11. **Get number of investors for ideas for one owner - check does it need pagination**\
```near view $NEARID get_investor_count_for_owner '{"owner_id":"'$NEARID'"}'``` 

11. **Get number of succesful phases and ideas for owner**\
```near view $NEARID count_phases_and_ideas_by_owner_id '{"owner_id":"'$NEARID'"}'``` 

TODO:
12. **Collect investment**\
```near call $NEARID collect_funds '{"idea_id":17, "project_phase":1}' --accountId $NEARID```

13. **Return investment if not realized**\
```near view $NEARID get_all_ideas```

**EXTERNAL TOKENS:**\
1. **Get the balance of the token sent to contract and connect it to idea**\
```near call $NEARID balance_of '{"contract_id":"dev-1674124791270-69359585766364","account_id": "'$NEARID'", "idea_id":3}'```



### FRONTEND


1. **Switch to frontend POC:**\
   ```cd frontend-poc```

In the project directory, you can run:

### `npm start`

Runs the app in the development mode.\
Open [http://localhost:3000](http://localhost:3000) to view it in your browser.# Dandelion2

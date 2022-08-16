Web3 JsRank
====================================================

Try to solve as many Javascript challenges as possible. 

The user's solution is uploaded into the smart contract and run several times with a different set of arguments. If the result of the user's code execution matches the pre-generated solution in all cases, then the code is correct.

User receives an NFT after solving any challenge, NFT shows actual Web3 JsRank for given user.

How to build
====================================================

For building the contract:

```
./buildanddeploy.sh
```

deploying it:

```
near dev-deploy
```

Run UI:

```
cd ui
yarn && yarn start
```

Based on
====

- [quickjs-rust-near](https://github.com/petersalomonsen/quickjs-rust-near) by Peter Salomonsen
- [HackerRank JS Challenges](https://www.hackerrank.com/challenges/)
- [near-create-app](https://github.com/near/create-near-app)


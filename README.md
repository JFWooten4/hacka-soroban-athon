## Stock Lending
Example implementation of a stock lending pool on Stellar, based on Block Transfer assets.

### Background
Stock lending is a practice that has traditionally been very centralized, with a small number of powerful brokers controlling access to the lending and borrowing system. This has led to a system in which most investors don't have access to stock lending, and the few that do often unknowingly pay a majority of their profits to these powerful brokers.

The history of share depositories dates back to the 19th century, with the formation of the Depository Trust Company (DTC) in the United States in 1973. The DTC serves as a central clearinghouse for the deposit and borrowing of shares among brokers. This has led all the brokers to deposit and borrow shares from each other through the DTC, allowing them to keep hundreds of millions in revenue from their clients' deposits.

However, this centralized system has a major disadvantage for individual investors. Most investors do not have access to stock lending, and the few that do often unknowingly pay a majority of their profits to powerful brokers. For example, Robinhood keeps 85% of revenue for users that opt into stock lending. This is possible because only a small number of powerful brokers have access to the lending and borrowing system.

### Motivation
But what if we could democratize the process? Imagine a system in which investors could earn passive yields just for maintaining their long-term portfolios. This is where decentralized stock lending comes in. By using smart contracts and blockchain technology, we can create a decentralized platform that allows investors to deposit and borrow shares directly from each other, without the need for powerful intermediaries. This would significantly lower the costs and increase access to stock lending for individual investors.

### This Project
In this project, we aim to create a decentralized stock lending protocol on the Stellar blockchain using Soroban. This protocol will allow investors to deposit shares into a pool, where other investors can borrow the shares and sell them short based on putting up cash reserves in the amount of the borrowed shares. The proceeds will be held in the loan pool until the position gets covered and profits distributed. Short investors will automatically get liquidated if they lose 60% on their short position, short investors have the option of adding more margin dollars to their position, depositors can withdraw available shares to sell at any time, and if there aren't enough shares to honor a deposit than the oldest borrower buys back their short position and returns the stock with the dollars held on their behalf.

The protocol also allows for an interest rate calculation that takes into account the supply and demand for the shares being borrowed. This ensures that the borrowing interest rate is fair and reflective of the current market conditions. Additionally, the protocol includes a function that allows for the distribution of profits among depositors and borrowers when the position is closed. This function takes into account the accumulated interest and the number of days the position was open.

### Impact
This decentralized stock lending protocol has the potential to democratize the stock lending process and give individual investors access to the same opportunities that were previously only available to powerful brokers. By using smart contracts and blockchain technology, we can create a transparent and fair system that is accessible to all.

### Disclaimer
Code provided as-is and without warrenty for the purposes of the Hacka-Soroban-athon.
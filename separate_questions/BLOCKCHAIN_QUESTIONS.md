## Questions

1. explain some of the ways hashing functions enable blockchain technology
2. briefly explain Bitcoin's UTXO model of transaction validation (separate from POW)
3. what is the structure of a Block in bitcoin and how does it relate to the 'blockchain' (merkle tree vs merkle list of merkle trees)
4. what problem/s are POW/POS trying to solve? discuss/compare (byzantine fault tolerance, reaching a single consensus on a p2p network)

## Q1

Hashing in blockchain refers to the process of having an input item of whatever length reflecting an output item of a fixed length.
If we take the example of blockchain use in cryptocurrencies, transactions of varying lengths are run through a given hashing algorithm,
and all give an output that is of a fixed length.

Hash functions, unlike encryption functions, are one-way. This means that it is easy and fast to calculate its hash from the input value.
However, it is not possible to calculate the input value from the hash. 
Encryption, on the contrary, allows you to first encrypt the data, and then decrypt it.

Hashing is commonly used in the blockchain, as it is a fast and reliable way to secure data, as well as bring them to the same length,
which will unify the process of building "blocks" in the chain.

## Q2

Basically the structure of UTXO is next:
Varsion                                 4 bytes
Number of inputs                        1 byte
Prev tx hash (reversed)                 32 bytes
Prev trans index                        4 bytes
Script length                           1 byte
Script signature (Unlocking script)     the script includes the amount of bytes (the first byte of sig)
Sequence                                4 bytes
Number of outputs                       1 byte
Value                                   8 bytes
Script Length                           1 byte
Script public key (Locking script)      the length of this is field above
Locking time                            4 bytes

`locking script` – a snippet of code in the Bitcoin Script language – 
dictates the type of authorization that is needed for a future transaction to spend this new bitcoin.

`unlocking script` - a snippet of code in the Bitcoin Script language.

The `unlocking script` of the Input segment of a transaction interacts with the locking script of the 
Output segment from a previous transaction.
The `locking script(s)` in the same transaction interacts with the unlocking script(s) in a future transaction.

When a UTXO is created via a transaction, it will incorporate a locking script. The locking script imposes a condition 
that must be met before the UTXO can be spent. If this UTXO is subsequently used as the Input in a future transaction, 
the Input segment of this future transaction must provide the unlocking script that will make the UTXO spendable, and 
consequently the transaction itself a valid transaction.

The transaction verification software in a full node copies the unlocking script from the Input segment.
Then, the verification software will use the first two fields of the Input segment to identify and retrieve the 
locking script and value of the Input UTXO.

## Q3

The block is made of a header, containing metadata, followed by a long list of transactions that make up the bulk of its size.
The block header is 80 bytes, whereas the average transaction is at least 250 bytes and the average block contains more than 500 transactions.

4 bytes             Block Size      The size of the block, in bytes, following this field
80 bytes            Block Header    Several fields form the block header
1-9 bytes (VarInt)  Transaction     Counter How many transactions follow
Variable            Transactions    The transactions recorded in this block


**_Block Header_**
The block header consists of three sets of block metadata. 
1. there is a reference to a previous block hash, which connects this block to the previous block in the blockchain. 
2. the difficulty, timestamp, and nonce, relate to the mining competition. 
3. the merkle tree root, a data structure used to efficiently summarize all the transactions in the block.

4 bytes     Version                 A version number to track software/protocol upgrades
32 bytes    Previous Block Hash     A reference to the hash of the previous (parent) block in the chain
32 bytes    Merkle Root             A hash of the root of the merkle tree of this block’s transactions
4 bytes     Timestamp               The approximate creation time of this block (seconds from Unix Epoch)
4 bytes     Difficulty              Target The proof-of-work algorithm difficulty target for this block
4 bytes     Nonce                   A counter used for the proof-of-work algorithm


A **_merkle tree_**, also known as a _binary hash tree_, is a data structure used for efficiently summarizing
and verifying the integrity of large sets of data. Merkle trees are binary trees containing cryptographic hashes. \
Merkle trees are used in bitcoin to summarize all the transactions in a block, producing an overall digital fingerprint 
of the entire set of transactions, providing a very efficient process to verify whether a transaction is included in a block.
A Merkle tree is constructed by recursively hashing pairs of nodes until there is only one hash, called the root, or merkle root.
The cryptographic hash algorithm used in bitcoin’s merkle trees is SHA256 applied twice, also known as double-SHA256.\
**_Note_**: When `N` data elements are hashed and summarized in a merkle tree, you can check to see if any one data element
is included in the tree with at most `2*log2(N)` calculations, making this a very efficient data structure.

## Q4

### PoW

Proof-of-Work is a mechanism Bitcoin uses to regulate the creation of blocks and the state of the blockchain.
Proof-of-Work provides an objective way for all members of the Bitcoin network to agree on the state of the blockchain and all Bitcoin transactions.

Proof-of-Work forces miners to make trillions of numerical guesses in order to produce a valid block, 
and thanks to the difficulty adjustment, miners collectively find one block every 10 minutes on average.

Proof-of-Work is random and fair due to the strong randomness of the SHA-256 hash function which underlies the Proof-of-Work mechanism.
There are no complex governance algorithms controlling which miners find blocks or decide the rules.

### PoS

Proof-of-Stake is an alternative consensus mechanism to Proof-of-Work, developed and used by a few alternative cryptocurrencies.
In the Proof-of-Stake model, stakers - the PoS equivalent of miners - lock up funds in a special smart contract. 
Every time a new block is needed by the network, an algorithm grants a specific staker the opportunity to publish the next block.
The algorithm selects the staker via lottery, depending on each staker’s percentage of total staked funds. For example, 
if a single staker controls 30% of all funds staked on a given network, they have a 30% chance of mining the next block.

### PoS vs PoW

Proof-of-Stake was invented to improve upon the perceived downsides Proof-of-Work. 
Firstly, Proof-of-Stake does not require the immense amount of energy consumption required by Proof-of-Work, 
because coins are simply locked in a specific smart contract on the blockchain. 
For this reason, individuals who criticize Bitcoin’s energy consumption prefer Proof-of-Stake.

Proof-of-Stake advocates also claim that PoS is more economically secure than PoW, however, 
this has been debated back and forth to no conclusion. 
Additionally, since Bitcoin has never been hacked and there are relatively few decentralized PoS systems,
claims that PoS is more secure than PoW have remained purely theoretical and not based on any empirical evidence.

### Drawbacks of Proof-of-Work

#### Consumption of Energy

Proof-of-Stake was invented to eliminate the massive energy costs of a Proof-of-Work network.
On the other hand, we always should think about the value of Bitcoin’s ultimate security.
No cryptocurrency, fiat currency, or even commodity-based money can match the security and immutability of Bitcoin’s blockchain.
For high-value transactions, security and reliability are often more important than speed. 
Thus, Bitcoin’s energy consumption is well worth the cost, as indicated by the strong demand to transact on the Bitcoin blockchain.
No cryptocurrency which has attempted to replicate Bitcoin and eliminate its energy requirements has established sufficient 
security and decentralization, precisely because Proof-of-Work converts energy into security.

#### Hardware Requirements

Satisfying the Proof-of-Work requirement necessitates physical computers and intensive computation. 
To maximize the energy efficiency of this process, miners use specialized hardware rather than normal laptops and general
purpose computers, which are not capable of anything other than Bitcoin mining.

The fact that this hardware has only one use protects Bitcoin by discouraging attackers.
If an attacker wanted to execute a 51% attack on the network, they would have to purchase millions if not billions of 
dollars worth of ASICs, only to render them useless by destroying the Bitcoin network.

### Drawbacks of Proof-of-Stake

#### Problems with Governance

Proof-of-Stake systems grant control of the network to owners of the token. 
Those with large amounts of the token can influence the rules of the network. 
They are also able to accumulate more of the token simply by staking. 
This positive feedback loop can lead to centralization of staked funds in the hands of exchanges and large institutions who custody user funds.

This is not the case for Proof-of-Work systems. 
Bitcoin’s ruleset is controlled by nodes and miners, and no power over the network is given to bitcoin owners.

#### Vulnerability to Attack

Proof-of-Stake systems are vulnerable to centralization and capture because control of the network is determined solely by capital,
which is far more centralized than labor and cheap energy. 
In a PoS network worth $100 billion where 10% of tokens are staked, the $100 billion network can be taken over by any party 
able to allocate $10 billion. All the attacker would have to do is send $10 billion in tokens to a staking contract.

In a Proof-of-Work network, capital, labor, and efficient allocation of both are required to execute an attack on the network.
Attacking a network with $10 billion of security would require purchasing ASICs, acquiring space and energy contracts to mine 
at a larger scale than the entire network, and acquiring and deploying the labor to execute the attack. 
If such an attack were underway, the entire network would likely be made aware ahead of time by the immense demand for ASICs and electricity.

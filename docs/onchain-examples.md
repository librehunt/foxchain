# On-chain examples: addresses, public keys, transactions

But de ce document: fournir un catalogue d’exemples réels (on-chain) multi-chaînes, y compris des cas d’adresses EVM réutilisées sur plusieurs réseaux. Chaque élément listé ci-dessous existe réellement on-chain et peut être vérifié via les explorateurs indiqués.

Note: Le format est volontairement simple pour être facilement parsable.

---

chain: bitcoin
address: 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa
verify: https://www.blockchain.com/explorer/addresses/btc/1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa

chain: bitcoin
# Coinbase du bloc de genèse
tx: 4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b
verify: https://www.blockchain.com/explorer/transactions/btc/4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b

chain: bitcoin
# Adresse historique SatoshiDice
address: 1dice8EMZmqKvrGE4Qc9bUFf9PX3xaYDp
verify: https://www.blockchain.com/explorer/addresses/btc/1dice8EMZmqKvrGE4Qc9bUFf9PX3xaYDp

---

# Cas EVM multi-chaînes (même adresse sur plusieurs réseaux)
# Adresse burn standard, utilisée largement sur de nombreuses chaînes EVM

chain: ethereum
address: 0x000000000000000000000000000000000000dEaD
verify: https://etherscan.io/address/0x000000000000000000000000000000000000dEaD

chain: ethereum
# Exemple de transaction réelle vers l’adresse burn (récoltée via API publique)
tx: 0xcdf331416ac94df404cfa95b13ecd4b23b2b1de895c945e25ff1b557c597a64e
verify: https://etherscan.io/tx/0xcdf331416ac94df404cfa95b13ecd4b23b2b1de895c945e25ff1b557c597a64e

chain: bsc
address: 0x000000000000000000000000000000000000dEaD
verify: https://bscscan.com/address/0x000000000000000000000000000000000000dEaD

chain: polygon
address: 0x000000000000000000000000000000000000dEaD
verify: https://polygonscan.com/address/0x000000000000000000000000000000000000dEaD

chain: avalanche-c
address: 0x000000000000000000000000000000000000dEaD
verify: https://snowtrace.io/address/0x000000000000000000000000000000000000dEaD

chain: arbitrum-one
address: 0x000000000000000000000000000000000000dEaD
verify: https://arbiscan.io/address/0x000000000000000000000000000000000000dEaD

chain: optimism
address: 0x000000000000000000000000000000000000dEaD
verify: https://optimistic.etherscan.io/address/0x000000000000000000000000000000000000dEaD

chain: base
address: 0x000000000000000000000000000000000000dEaD
verify: https://basescan.org/address/0x000000000000000000000000000000000000dEaD

chain: fantom
address: 0x000000000000000000000000000000000000dEaD
verify: https://ftmscan.com/address/0x000000000000000000000000000000000000dEaD

chain: gnosis
address: 0x000000000000000000000000000000000000dEaD
verify: https://gnosisscan.io/address/0x000000000000000000000000000000000000dEaD

chain: zksync-era
address: 0x000000000000000000000000000000000000dEaD
verify: https://explorer.zksync.io/address/0x000000000000000000000000000000000000dEaD

chain: linea
address: 0x000000000000000000000000000000000000dEaD
verify: https://lineascan.build/address/0x000000000000000000000000000000000000dEaD

chain: scroll
address: 0x000000000000000000000000000000000000dEaD
verify: https://scrollscan.com/address/0x000000000000000000000000000000000000dEaD

chain: mantle
address: 0x000000000000000000000000000000000000dEaD
verify: https://mantlescan.xyz/address/0x000000000000000000000000000000000000dEaD

---

# Autres exemples utiles (EVM / tokens / services)

chain: ethereum
# Vitalik (adresse EOA bien connue)
address: 0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045
verify: https://etherscan.io/address/0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045

chain: ethereum
# USDT (Tether) contrat ERC-20
address: 0xdAC17F958D2ee523a2206206994597C13D831ec7
verify: https://etherscan.io/token/0xdAC17F958D2ee523a2206206994597C13D831ec7

chain: ethereum
# ENS Registry
address: 0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e
verify: https://etherscan.io/address/0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e

chain: ethereum
# Uniswap V2 Router
address: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
verify: https://etherscan.io/address/0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D

---

# Solana

chain: solana
# SPL Token Program (adresse de programme)
address: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
public_key: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
verify: https://explorer.solana.com/address/TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA

chain: solana
# Transaction réelle associée au programme SPL-Token
transaction: 5wpHU1gGYcgKabL7heGGgiKBx3WJMruHiN34sCjTYwQu4sk9H2uMyZsm1P28RqaJPVELtcVxNmSGieq6V5ZZxpDT
verify: https://explorer.solana.com/tx/5wpHU1gGYcgKabL7heGGgiKBx3WJMruHiN34sCjTYwQu4sk9H2uMyZsm1P28RqaJPVELtcVxNmSGieq6V5ZZxpDT

chain: solana
# Stake Program
address: Stake11111111111111111111111111111111111111
public_key: Stake11111111111111111111111111111111111111
verify: https://explorer.solana.com/address/Stake11111111111111111111111111111111111111

chain: solana
# Sysvar Rent
address: SysvarRent111111111111111111111111111111111
public_key: SysvarRent111111111111111111111111111111111
verify: https://explorer.solana.com/address/SysvarRent111111111111111111111111111111111

chain: solana
# USDC mint
address: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
public_key: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v
verify: https://explorer.solana.com/address/EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v

---

# Tron

chain: tron
# Adresse "blackhole"/foundation connue
address: T9yD14Nj9j7xAB4dbGeiX9h8unkKHxuWwb
verify: https://tronscan.org/#/address/T9yD14Nj9j7xAB4dbGeiX9h8unkKHxuWwb

chain: tron
# Transaction réelle vers l’adresse ci-dessus
transaction: 5156e18743c2ceba71f40640c75a8402066a8c42e570f17eecda2cc1101575f4
verify: https://tronscan.org/#/transaction/5156e18743c2ceba71f40640c75a8402066a8c42e570f17eecda2cc1101575f4

chain: tron
# USDT (TRC-20) contrat
address: TXLAQ63Xg1NAzckPwKHvzw7CSEmLMEqcdj
verify: https://tronscan.org/#/contract/TXLAQ63Xg1NAzckPwKHvzw7CSEmLMEqcdj

---

# XRP (Ripple)

chain: xrp
address: rEb8TK3gBgk5auZkwc6sHnwrGVJH8DuaLh
public_key: 03D583FDA231987F521FEE3DF235D88EC848EFA84BEF1927DC5791F09A8DB0E1C1
transaction: 3D635C4F3F44C7D13F0AA5F6C4182859D1E3242B8714D74C1E8073910691ACBF
verify: https://xrpscan.com/tx/3D635C4F3F44C7D13F0AA5F6C4182859D1E3242B8714D74C1E8073910691ACBF

---

# Tezos

chain: tezos
# Burn address (tz1burn...)
address: tz1burnburnburnburnburnburnburjAYjjX
transaction: opDf2CkvCRvoecZekrf8CYD2Dv7zdX93Dhr1zJcJvPSu9XfZnZY
verify: https://tzkt.io/opDf2CkvCRvoecZekrf8CYD2Dv7zdX93Dhr1zJcJvPSu9XfZnZY

---

# Cardano (ADA)

chain: cardano
# Transaction réelle (dernière génération de bloc)
transaction: e2e3f3dd50250a7d76e672b80a96a420693cd21f4678a6c586101e9365618fdb
verify: https://cardanoscan.io/transaction/e2e3f3dd50250a7d76e672b80a96a420693cd21f4678a6c586101e9365618fdb

---

# NEAR

chain: near
address: vladyazh.near
public_key: ed25519:7HQsFwSMyTVC5F6oW74bFPCfhnmAHE5zSMZDKiP4EHk
transaction: 9XGBYcsj1eDWzD72Acm9xSJdcHFw4rYJKewLXH8aj3Zt
verify: https://nearblocks.io/txns/9XGBYcsj1eDWzD72Acm9xSJdcHFw4rYJKewLXH8aj3Zt

---

# Cosmos SDK (Cosmos Hub / Osmosis / Juno)

chain: cosmoshub
# Tx réelle (première tx du dernier bloc)
transaction: 3e0ba99f9a254b4dec6ee5cb04f833535dd409eccc26133d8df0cf943ee9b326
verify: https://www.mintscan.io/cosmos/txs/3e0ba99f9a254b4dec6ee5cb04f833535dd409eccc26133d8df0cf943ee9b326

chain: osmosis
# Tx réelle (première tx du dernier bloc)
transaction: c397c9c465418c9840139d95459512a9cbf705d57954ab20f709bca1043f7b63
verify: https://www.mintscan.io/osmosis/txs/c397c9c465418c9840139d95459512a9cbf705d57954ab20f709bca1043f7b63

chain: juno
# Tx réelle (première tx du dernier bloc)
transaction: af000fd46500e8c9f06ef2731c4d5d6b084570090abd33a71eaae37232acba78
verify: https://www.mintscan.io/juno/txs/af000fd46500e8c9f06ef2731c4d5d6b084570090abd33a71eaae37232acba78

---

# Substrate (Polkadot / Kusama)

chain: polkadot
# Extrinsic réel (dernier bloc, index 0)
extrinsic: 28815161-0
verify: https://polkadot.subscan.io/extrinsic/28815161-0

chain: kusama
# Extrinsic réel (dernier bloc, index 0)
extrinsic: 31206697-0
verify: https://kusama.subscan.io/extrinsic/31206697-0

---

# Stellar

chain: stellar
address: GBFXAEHTRZHLP4COKJSW72L26NLBEZLM3NQAD5F32SBFE6VJUVEQSETF
transaction: 26be97ef5f49f6330e19aa16450cdead05e044bced515c0c721a9a47a68626cb
verify: https://stellar.expert/explorer/public/tx/26be97ef5f49f6330e19aa16450cdead05e044bced515c0c721a9a47a68626cb

---

# Litecoin

chain: litecoin
# Coinbase (exemple de tx du bloc courant)
transaction: 1000bde6f051bfb65ab62ea8a596f4b3cd5fb0768c3c23e26c474287c1c1490f
verify: https://litecoinspace.org/tx/1000bde6f051bfb65ab62ea8a596f4b3cd5fb0768c3c23e26c474287c1c1490f

---

# Dogecoin

chain: dogecoin
# Exemple de tx tirée du dernier bloc consulté
transaction: dbd12217b1c6ca2d666ccde0d023bc5ff1284ab09d3ec7ffbc5d7f98db3d233d
verify: https://blockchair.com/dogecoin/transaction/dbd12217b1c6ca2d666ccde0d023bc5ff1284ab09d3ec7ffbc5d7f98db3d233d

chain: dogecoin
# Adresse de donation connue
address: DH5yaieqoZN36fDVciNyRueRGvGLR3mr7L
verify: https://blockchair.com/dogecoin/address/DH5yaieqoZN36fDVciNyRueRGvGLR3mr7L

---

# Aptos

chain: aptos
# Transaction récente (block epilogue)
transaction: 0x128d3d136602723ab5615357b84d4f913f2f3bac266b9d9d0e041121ad8d4e9e
verify: https://explorer.aptoslabs.com/txn/0x128d3d136602723ab5615357b84d4f913f2f3bac266b9d9d0e041121ad8d4e9e?network=mainnet

---

# Sui

chain: sui
# Transaction récente (digest)
transaction: 7CuBm1AnLgkBMB6GiEn5d3RizznF5LbawjJTs8A5dcXF
verify: https://suiscan.xyz/mainnet/tx/7CuBm1AnLgkBMB6GiEn5d3RizznF5LbawjJTs8A5dcXF

---

# Cardano (supplément)

chain: cardano
transaction: 0f63781afc7a80d7ddbf4f6e31c94809a04ce95821009a42c47dd5b6916e674e
verify: https://cardanoscan.io/transaction/0f63781afc7a80d7ddbf4f6e31c94809a04ce95821009a42c47dd5b6916e674e

chain: cardano
transaction: e91e76fb15f69115d8a9835bee0e7b82548190d25d3998e473a7a48d734fd6ab
verify: https://cardanoscan.io/transaction/e91e76fb15f69115d8a9835bee0e7b82548190d25d3998e473a7a48d734fd6ab

---

# Litecoin (supplément)

chain: litecoin
transaction: 8cd6f04201ff962f7e311f89612976e27338e214a00c0087ff5f8812336dde1d
verify: https://litecoinspace.org/tx/8cd6f04201ff962f7e311f89612976e27338e214a00c0087ff5f8812336dde1d

chain: litecoin
transaction: 89002e164c6315bcd628f2dacfb1060dff42b80e0d4b1b09271203626c6cbf4a
verify: https://litecoinspace.org/tx/89002e164c6315bcd628f2dacfb1060dff42b80e0d4b1b09271203626c6cbf4a

chain: litecoin
address: LcNS6c8RddAMjewDrUAAi8BzecKoosnkN3
verify: https://litecoinspace.org/address/LcNS6c8RddAMjewDrUAAi8BzecKoosnkN3

---

# Sui (supplément)

chain: sui
transaction: 2JxYGfYyqvUCJLtkqF71oVFQVk7CasppV6BswFW8J2vs
verify: https://suiscan.xyz/mainnet/tx/2JxYGfYyqvUCJLtkqF71oVFQVk7CasppV6BswFW8J2vs

chain: sui
transaction: 9qH4LRaMcmyTUobd9nvU3LbxfXGE5kqS2QGgk86zH49g
verify: https://suiscan.xyz/mainnet/tx/9qH4LRaMcmyTUobd9nvU3LbxfXGE5kqS2QGgk86zH49g

---

# Notes
- Toutes les adresses/transactions ci-dessus sont réelles et vérifiables via les liens d’explorateurs fournis.
- Les adresses EVM (format 0x…) sont valides sur la plupart des L1/L2 compatibles EVM: la même clé privée dérive la même adresse sur Ethereum, BSC, Polygon, Arbitrum, Optimism, Avalanche C-Chain, Base, Fantom, Gnosis, zkSync Era, Linea, Scroll, Mantle, etc. L’activité (soldes/tx) peut néanmoins varier selon la chaîne.
- Pour enrichir ce catalogue (Substrate/Polkadot, Cardano adresses, etc.), ajouter d’autres entrées au même format en veillant à référencer des éléments réellement on-chain, idéalement avec un lien d’explorateur.

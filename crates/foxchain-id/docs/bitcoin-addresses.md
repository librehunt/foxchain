# Bitcoin Ecosystem Address Format

Bitcoin and related chains (Litecoin, Dogecoin) use multiple address formats: P2PKH (legacy), P2SH (script hash), and Bech32 (native SegWit).

## Format Overview

Bitcoin addresses come in three main formats:

1. **P2PKH (Pay-to-PubKey-Hash)**: Legacy format, starts with `1`
2. **P2SH (Pay-to-Script-Hash)**: Script hash format, starts with `3`
3. **Bech32**: Native SegWit format, starts with `bc1` (Bitcoin) or `lt1` (Litecoin)

## P2PKH Addresses (Legacy)

### Format Specification

- **Prefix**: `1` (Bitcoin mainnet)
- **Encoding**: Base58Check
- **Length**: 25 bytes when decoded (1 byte version + 20 bytes hash + 4 bytes checksum)
- **Display length**: Typically 26-35 characters

### Encoding Process

1. Take the 20-byte hash160 (RIPEMD160(SHA256(public key)))
2. Prepend version byte (0x00 for Bitcoin mainnet)
3. Append 4-byte checksum (first 4 bytes of SHA256(SHA256(version + hash)))
4. Encode entire 25-byte result in Base58

### Example

```
1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa  (Genesis block address)
```

## P2SH Addresses (Script Hash)

### Format Specification

- **Prefix**: `3` (Bitcoin mainnet)
- **Encoding**: Base58Check
- **Length**: 25 bytes when decoded (1 byte version + 20 bytes hash + 4 bytes checksum)
- **Version byte**: 0x05 for Bitcoin mainnet

### Use Cases

- Multi-signature wallets
- Complex scripts
- SegWit wrapped addresses (P2SH-P2WPKH, P2SH-P2WSH)

### Example

```
3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy
```

## Bech32 Addresses (Native SegWit)

### Format Specification

- **Prefix**: `bc1` (Bitcoin mainnet), `tb1` (testnet), `ltc1` (Litecoin), `lt1` (Litecoin)
- **Encoding**: Bech32
- **Length**: Variable (typically 42-62 characters)
- **Structure**: `{hrp}1{data}` where HRP is human-readable part

### Bech32 Encoding

Bech32 uses a 5-bit encoding scheme:

1. Convert 8-bit bytes to 5-bit groups
2. Append checksum (6 characters)
3. Encode using character set: `qpzry9x8gf2tvdw0s3jn54khce6mua7l`

### Benefits

- **Error detection**: Built-in checksum prevents typos
- **Case insensitive**: Can be written in any case
- **Lower fees**: Native SegWit transactions are cheaper
- **Future-proof**: Supports larger witness programs

### Example

```
bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4  (P2WPKH)
bc1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3qccfmv3  (P2WSH)
```

## Base58Check Encoding

Base58Check is used for P2PKH and P2SH addresses.

### Character Set

```
123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz
```

Note: Excludes `0`, `O`, `I`, `l` to avoid confusion.

### Checksum Calculation

1. Take payload (version byte + hash)
2. Compute `SHA256(SHA256(payload))`
3. Take first 4 bytes as checksum
4. Append checksum to payload
5. Encode in Base58

### Validation

1. Decode Base58 string
2. Extract checksum (last 4 bytes)
3. Verify checksum matches `SHA256(SHA256(payload))[0:4]`

## Chain Identification

Different chains use different version bytes:

| Chain | P2PKH Version | P2SH Version | Bech32 HRP |
|-------|---------------|--------------|------------|
| Bitcoin | 0x00 | 0x05 | `bc1` |
| Litecoin | 0x30 | 0x32 | `ltc1` or `lt1` |
| Dogecoin | 0x1e | 0x16 | N/A |
| Testnet | 0x6f | 0xc4 | `tb1` |

## Validation Rules

### P2PKH/P2SH

- ✅ Starts with correct prefix (`1` or `3` for Bitcoin)
- ✅ Valid Base58Check encoding
- ✅ Correct length (25 bytes when decoded)
- ✅ Valid checksum

### Bech32

- ✅ Starts with correct HRP (`bc1`, `ltc1`, etc.)
- ✅ Valid Bech32 encoding
- ✅ Valid checksum
- ✅ Correct witness program length

## Normalization

`foxchain-id` normalizes Bitcoin addresses:

- **P2PKH/P2SH**: Keep as-is (Base58Check is canonical)
- **Bech32**: Convert to lowercase (Bech32 is case-insensitive, lowercase is standard)

### Preferred Format

For new addresses, Bech32 (native SegWit) is preferred:
- Lower transaction fees
- Better error detection
- Future-proof design

## Multi-chain Considerations

The Bitcoin ecosystem includes multiple chains:

- **Bitcoin**: Original chain, most common
- **Litecoin**: Similar format, different version bytes
- **Dogecoin**: Similar format, different version bytes

Addresses can be distinguished by:
- Version byte (for P2PKH/P2SH)
- HRP prefix (for Bech32)

## Implementation Details

### Detection Algorithm

1. Check if address starts with `1` or `3` (P2PKH/P2SH)
2. Validate Base58Check encoding
3. Verify checksum
4. Identify chain from version byte
5. OR check if address starts with known Bech32 HRP
6. Validate Bech32 encoding and checksum
7. Identify chain from HRP

### Planned Support

- Bitcoin (P2PKH, P2SH, Bech32)
- Litecoin (P2PKH, P2SH, Bech32)
- Dogecoin (P2PKH, P2SH)

## Examples

### Valid Addresses

```
# Bitcoin P2PKH
1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa

# Bitcoin P2SH
3J98t1WpEZ73CNmQviecrnyiWrnqRhWNLy

# Bitcoin Bech32
bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4

# Litecoin P2PKH
LTC1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa

# Litecoin Bech32
ltc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4
```

## Technical References

- [BIP 13: Address Format for pay-to-script-hash](https://github.com/bitcoin/bips/blob/master/bip-0013.mediawiki)
- [BIP 173: Base32 address format for native v0-16 witness outputs](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki)
- [Base58Check Encoding](https://en.bitcoin.it/wiki/Base58Check_encoding)
- [Bech32 Specification](https://github.com/bitcoin/bips/blob/master/bip-0173.mediawiki#Bech32)

## See Also

- [EVM Addresses](evm-addresses.md) - Different format for Ethereum and EVM chains
- [Solana Addresses](solana-addresses.md) - Base58 encoding used by Solana
- [Cosmos Addresses](cosmos-addresses.md) - Bech32 encoding with HRP

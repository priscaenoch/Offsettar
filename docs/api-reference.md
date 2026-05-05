# API Reference

Base URL: `https://api.offsetta.xyz/v1`

All requests require the header: `X-API-Key: <your_api_key>`

---

## Onramp

### POST /onramp/deposit

Initiate a mobile money → USDC deposit.

**Request**
```json
{
  "amount": 1000,
  "currency": "KES",
  "mobileNumber": "+254700000000",
  "provider": "mpesa",
  "stellarAddress": "<stellar_address>"
}
```

**Response**
```json
{
  "depositId": "dep_xxx",
  "status": "pending",
  "expiresAt": "2024-01-01T00:05:00Z"
}
```

---

## Offramp

### POST /offramp/withdraw

Initiate a USDC → mobile money withdrawal.

**Request**
```json
{
  "amount": 10,
  "stellarAddress": "<stellar_address>",
  "mobileNumber": "+254700000000",
  "provider": "mpesa"
}
```

---

## Routing

### POST /routing/quote

Get a fee + rate quote for a corridor.

**Request**
```json
{
  "fromCurrency": "KES",
  "toCurrency": "USDC",
  "amount": 1000,
  "corridor": "KE"
}
```

**Response**
```json
{
  "estimatedAmount": 7.42,
  "fee": 0.03,
  "rate": 0.00745,
  "expiresAt": "2024-01-01T00:01:00Z"
}
```

---

## Liquidity

### GET /liquidity/pools

Returns all active liquidity pools and their balances.

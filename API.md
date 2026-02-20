# URL Shortener API

A lightweight URL shortening service built with Rust (Axum) and PostgreSQL.

**Base URL:** `http://localhost:4000`

---

## Endpoints

### 1. Shorten a URL

Creates a shortened URL from a given original URL.

```
POST /api/shorten
```

**Request Body**

| Field | Type   | Required | Description             |
| :---- | :----- | :------- | :---------------------- |
| `url` | string | Yes      | The original URL to shorten |

```json
{
  "url": "https://example.com/very/long/url/path"
}
```

**Response `201 Created`**

| Field        | Type   | Description                        |
| :----------- | :----- | :--------------------------------- |
| `short_code` | string | Generated 6-character alphanumeric code |
| `short_url`  | string | Complete shortened URL             |

```json
{
  "short_code": "aBc123",
  "short_url": "http://localhost:4000/aBc123"
}
```

**Status Codes**

| Code  | Description                         |
| :---- | :---------------------------------- |
| `201` | Short URL successfully created      |
| `500` | Database error or code collision    |

---

### 2. Redirect to Original URL

Redirects to the original URL and increments the visit counter.

```
GET /:code
```

**Path Parameters**

| Parameter | Type   | Description                  |
| :-------- | :----- | :--------------------------- |
| `code`    | string | The short code to resolve    |

**Response `302 Found`**

Returns an HTTP redirect. The `Location` header is set to the original URL.

```
Location: https://example.com/very/long/url/path
```

**Status Codes**

| Code  | Description                         |
| :---- | :---------------------------------- |
| `302` | Redirect to original URL            |
| `404` | Short code not found                |
| `500` | Database error                      |

---

### 3. Get URL Statistics

Returns metadata and visit statistics for a short code.

```
GET /api/stats/:code
```

**Path Parameters**

| Parameter | Type   | Description                  |
| :-------- | :----- | :--------------------------- |
| `code`    | string | The short code to look up    |

**Response `200 OK`**

| Field          | Type    | Description                              |
| :------------- | :------ | :--------------------------------------- |
| `original_url` | string  | The original long URL                    |
| `visit_count`  | integer | Number of times the short URL was visited |
| `created_at`   | string  | ISO 8601 creation timestamp              |

```json
{
  "original_url": "https://example.com/very/long/url/path",
  "visit_count": 42,
  "created_at": "2024-01-15T10:30:45.123456Z"
}
```

**Status Codes**

| Code  | Description                         |
| :---- | :---------------------------------- |
| `200` | Stats returned successfully         |
| `404` | Short code not found                |
| `500` | Database error                      |

---

## Endpoint Summary

| Method | Path               | Description              | Auth |
| :----- | :----------------- | :----------------------- | :--- |
| POST   | `/api/shorten`     | Create a short URL       | None |
| GET    | `/:code`           | Redirect to original URL | None |
| GET    | `/api/stats/:code` | Get URL statistics       | None |

---

## Database Schema

**Table: `urls`**

| Column        | Type          | Constraints  | Default |
| :------------ | :------------ | :----------- | :------ |
| `id`          | VARCHAR(10)   | PRIMARY KEY  | —       |
| `original_url`| TEXT          | NOT NULL     | —       |
| `created_at`  | TIMESTAMPTZ   | NOT NULL     | `NOW()` |
| `visit_count` | BIGINT        | NOT NULL     | `0`     |

---

## Configuration

| Environment Variable | Required | Description                         |
| :------------------- | :------- | :---------------------------------- |
| `DATABASE_URL`       | Yes      | PostgreSQL connection string (`postgres://user:password@host:port/db`) |
| `PORT`               | No       | Listening port (default: `4000`)    |

---

## Known Limitations

- Short codes are hardcoded to resolve against `http://localhost:4000` — no configurable base URL.
- No URL format validation on input.
- No collision retry logic on code generation.
- No authentication or rate limiting on any endpoint.
- Database errors are returned as raw strings.

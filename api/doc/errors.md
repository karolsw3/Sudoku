# Errors
Requests can fail, here's how they can fail.

## Generic error

Used for most non-specific errors.

`reason` should be in an all-lowercase past-tense finishing-punctuation-free form (e.g. "failed to apply diff", "user with that name exists"),
  `severity` is an enum, wherein `"warning"` indicates a non-fatal error, which is usually a user's fault, and `"danger"` indicates a potentially-fatal,
  usually internal error or a malicious request.

```json
{
    "reason": "string",
    "severity": "warning | danger"
}
```

## Login error

<sub>&lt;secure&gt;</sub>

```json
{}
```

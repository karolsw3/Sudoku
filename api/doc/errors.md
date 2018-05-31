# Errors
Requests can fail, here's how they can fail.

## Generic error

Used for most non-specific errors.

`reason` should be in an all-lowercase past-tense finishing-punctuation-free form (e.g. "failed to apply diff", "user with that name exists").

```json
{
    "reason": "string"
}
```

## Login error

<sub>&lt;secure&gt;</sub>

```json
{}
```

# brag-server

The value returned for each day is a sum of the:

- Number of commits in all registered repositories for the selected git contributors

Data schema for a basic request:

```json
{
  "data": [
    { date: '2023-10-01', value: 3 },
    { date: '2023-10-02', value: 30 },
    { date: '2023-08-02', value: 45 },
    { date: '2023-09-02', value: 60 },
    { date: '2023-07-15', value: 1 }
  ]
}
```

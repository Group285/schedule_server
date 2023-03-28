# Документация по api

## Условные обозначения

### Условное обозначение вызова API запроса

[Стандартный формат HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages)

Типы данных оборачиваются в ${}, например: ${String}

#### Пример

- Запросы

### Условные обозначения типов

- int - целочисленный тип(64 бита)
- float - число с целочисленной дробью
- String - строка
- Остальные типы представляют собой сложные структуры, которые будут описаны в json стиле, например:

```http
Schedule
{
    "name": String,
    "time": int,
}
```

## API

### User

- тип User

```json
{
    "_id": String,
    "username": String,
    "admin": bool
}
```

- Добавить пользователя

```http
POST /user
Cookie: uid_schedule_token=${String}

${User}
```

- Удалить пользователя

```http
DELETE /user/${String}
Cookie: uid_schedule_token=${String}
```

- Обновить пользователя

```http
PUT /user
Cookie: uid_schedule_token=${String}

${User}
```

# Документация по api

## Условные обозначения

### Условное обозначение вызова API запроса

[Стандартный формат HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP/Messages)

Типы данных оборачиваются в \${}, например: \${str}

### Условные обозначения типов

- int - целочисленный тип(64 бита)
- float - число с целочисленной дробью
- str - строка
- bool - булево значение(true или false)

Остальные типы представляют собой сложные структуры, которые будут описаны в json стиле, например:

    {
        "name": str,
        "time": int,
    }

## API

### User

#### Тип User

    {
        "_id": str,
        "username": str,
        "admin": bool
    }

---

- Добавить пользователя
  - Запрос

        POST /user
        Cookie: uid_schedule_token=${str}

        ${User}

  - Ответ
    - Если логин имеет права доступа и пользователь валиден

            HTTP/1.1 200 OK

    - Если логин не имеет прав доступа

            HTTP/1.1 401 UNAUTHORIZED

    - Если пользователь не валиден

            TODO

- Удалить пользователя
  - Запрос

        DELETE /user/${str}
        Cookie: uid_schedule_token=${str}

  - Ответ

    - Если логин имеет права доступа и uid пользователя существует

            HTTP/1.1 200 OK

    - Если логин не имеет прав доступа

            HTTP/1.1 401 UNAUTHORIZED

    - Если uid пользователя не существует

            HTTP/1.1 404 NOT_FOUND

- Обновить пользователя
  - Запрос

        PUT /user
        Cookie: uid_schedule_token=${str}

        ${User}

  - Ответ
    - Если логин имеет права доступа и uid пользователя существует

            HTTP/1.1 200 OK

    - Если логин не имеет прав доступа

            HTTP/1.1 401 UNAUTHORIZED

    - Если uid пользователя не существует

            HTTP/1.1 404 NOT_FOUND

### Mark

#### Тип Mark

    {
        "_id": int,
        "lesson_id": int,
        "user_id": str,
        "mark": str
    }

---

- Добавить оценку
  - Запрос

        POST /mark
        Cookie: uid_schedule_token=${str}

        ${Mark}

  - Ответ
    - Если логин имеет права доступа и оценка валидна

            HTTP/1.1 200 OK

    - Если логин не имеет прав доступа

            HTTP/1.1 401 UNAUTHORIZED

    - Если оценка не валидна

            TODO

- Удалить оценку
  - Запрос

        DELETE /mark/${str}
        Cookie: uid_schedule_token=${str}

  - Ответ

    - Если логин имеет права доступа и id оценки существует

            HTTP/1.1 200 OK

    - Если логин не имеет прав доступа

            HTTP/1.1 401 UNAUTHORIZED

    - Если id оценки не существует

            HTTP/1.1 404 NOT_FOUND

- Обновить оценку
  - Запрос

        PUT /mark
        Cookie: uid_schedule_token=${str}

        ${Mark}

  - Ответ
    - Если логин имеет права доступа и id оценки существует

            HTTP/1.1 200 OK

    - Если логин не имеет прав доступа

            HTTP/1.1 401 UNAUTHORIZED

    - Если id оценки не существует

            HTTP/1.1 404 NOT_FOUND

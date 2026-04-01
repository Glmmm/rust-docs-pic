# Documento Base

Lorem ipsum dolor sit amet consectetur, adipisicing elit. Voluptas autem ab tenetur aspernatur, illo neque earum incidunt itaque qui, consequatur sit. Enim facere totam ea nisi optio earum ab nesciunt?

## Diagrama de Classe

Lorem ipsum dolor sit amet consectetur, adipisicing elit. Voluptas autem ab tenetur aspernatur, illo neque earum incidunt itaque qui, consequatur sit. Enim facere totam ea nisi optio earum ab nesciunt?

## Variáveis encontradas

Lorem ipsum dolor sit amet consectetur, adipisicing elit. Voluptas autem ab tenetur aspernatur, illo neque earum incidunt itaque qui, consequatur sit. Enim facere totam ea nisi optio earum ab nesciunt?

```php
string $name,
mixed $connection,
?array $with = null,
?array $additional = null
```

## Funções encontradas

Lorem ipsum dolor sit amet consectetur, adipisicing elit. Voluptas autem ab tenetur aspernatur, illo neque earum incidunt itaque qui, consequatur sit. Enim facere totam ea nisi optio earum ab nesciunt?

<details>
<summary>Código completo</summary>

```php
<?php
namespace LucasGenerozo\Migrator\Models\Domain\Database;

use LucasGenerozo\Migrator\Models\Domain\Database\DatabaseType;
use LucasGenerozo\Migrator\Models\Domain\DataSource\DataSource;

interface Database
{
    public function __construct(
        ?int $id,
        DatabaseType $type,
        string $name,
        array $options,
    );

    public function setId(?int $id): void;
    public function getId(): ?int;
    public function getDataSource(string $name, ?array $with = []): ?DataSource;
    public function toArray(): array;
    public function listDataSources(): array;

}
```

</details>

```php
//(comentário caso exista)
 __construct(string $name, mixed $connection, ?array $with = null, ?array $additional = null) : DataSource
```

```php
//(comentário caso exista)
 __construct(string $name) : DataSource
```

```php
//(comentário caso exista)
 __construct(string $name, mixed $connection) : DataSource
```

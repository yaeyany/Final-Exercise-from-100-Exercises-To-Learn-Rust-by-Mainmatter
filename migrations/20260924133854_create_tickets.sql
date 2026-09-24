CREATE TABLE tickets (

    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    title TEXT NOT NULL,

    description TEXT,

    priority TEXT NOT NULL
        CHECK (priority IN ('low', 'medium', 'high')),

    status TEXT NOT NULL
        CHECK (status IN ('new', 'in progress', 'completed'))
);

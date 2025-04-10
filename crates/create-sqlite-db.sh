SQLQUERY=$(cat <<-END
CREATE TABLE IF NOT EXISTS todos
(
    id          INTEGER PRIMARY KEY NOT NULL,
    description TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);
END
)
docker run -ti --rm -v $(pwd):/apps -w /apps alpine/sqlite todos.db "$SQLQUERY"

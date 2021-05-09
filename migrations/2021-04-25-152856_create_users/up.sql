-- Your SQL goes here
CREATE TABLE users
(
    id         int PRIMARY KEY AUTO_INCREMENT                                  NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP                             NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP,
    user_name  VARCHAR(50)                                                     NOT NULL,
    first_name VARCHAR(50)                                                     NOT NULL,
    last_name  VARCHAR(50),
    pin        VARCHAR(255)                                                    NOT NULL
);
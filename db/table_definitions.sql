-- RunnerVMs Table
CREATE TABLE RunnerVMs (
    Id TEXT PRIMARY KEY NOT NULL,
    Status TEXT CHECK(Status IN ('RESETTING', 'IDLE', 'RUNNING', 'ERROR', 'OFFLINE')) NOT NULL,
    TimeToReset TIMESTAMP
);

-- Hardware Table
CREATE TABLE Hardware (
    Id TEXT PRIMARY KEY NOT NULL,
    Status TEXT CHECK(Status IN ('FREE', 'CLAIMED', 'UNAVAILABLE', 'ERROR')) NOT NULL,
    ClaimedBy Name TEXT,
    FOREIGN KEY (ClaimedBy) REFERENCES RunnerVMs (Id)
);


-- Runners
INSERT INTO RunnerVMs (ID, Status, TimeToReset) VALUES
('Runner1', 'RESETTING', NULL),
('Runner2', 'IDLE', NULL);

-- Hardware
INSERT INTO Hardware (ID, Status, ClaimedBy) VALUES
('Rpi4', 'FREE', NULL),
('Rpi3', 'FREE', NULL);

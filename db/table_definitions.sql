-- RunnerVMs Table
CREATE TABLE RunnerVMs (
    VM_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    Name TEXT NOT NULL,
    Status TEXT CHECK(Status IN ('RESETTING', 'IDLE', 'RUNNING', 'ERROR')) NOT NULL,
    TimeToReset TIMESTAMP
);

-- Hardware Table
CREATE TABLE Hardware (
    Board_ID INTEGER PRIMARY KEY AUTOINCREMENT,
    Name TEXT NOT NULL,
    Status TEXT CHECK(Status IN ('FREE', 'CLAIMED', 'UNAVAILABLE')) NOT NULL,
    ClaimedBy INTEGER,
    FOREIGN KEY (ClaimedBy) REFERENCES RunnerVMs (VM_ID)
);


-- Runners
INSERT INTO RunnerVMs (Name, Status, TimeToReset) VALUES
('Runner1', 'RESETTING', NULL);

-- Hardware
INSERT INTO Hardware (Name, Status, ClaimedBy) VALUES
('Rpi4', 'FREE', NULL),
('Rpi3', 'FREE', NULL);

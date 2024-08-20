-- Add migration script here
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
('ulmc-trnt-app02', 'RESETTING', NULL);
--('ulmc-trnt-app03', 'IDLE', NULL);

-- Hardware
INSERT INTO Hardware (ID, Status, ClaimedBy) VALUES
('rpi4', 'FREE', NULL),
('rpi3', 'FREE', NULL),
('nitrogen6sx', 'FREE', NULL),
('sabre', 'FREE', NULL),
('odroidc2', 'FREE', NULL);
--('jetson-nano-2gb-dev-kit', 'FREE', NULL),
--('jetson-tx2-nx-a206', 'FREE', NULL),
--('jetson-xavier-nx-dev-kit', 'FREE', NULL),
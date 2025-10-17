-- Table principale bâtiments
CREATE TABLE IF NOT EXISTS buildings (
    id BIGINT PRIMARY KEY,
    chunk_x INT NOT NULL,
    chunk_y INT NOT NULL,
    coord_q INT NOT NULL,
    coord_r INT NOT NULL,
    
    category VARCHAR(32) NOT NULL,
    variant VARCHAR(64) NOT NULL,
    
    health REAL NOT NULL,
    max_health REAL NOT NULL,
    construction_progress REAL DEFAULT 1.0,
    owner_id BIGINT,
    
    metadata JSONB NOT NULL,
    
    created_at BIGINT NOT NULL,
    last_modified BIGINT NOT NULL,
    
    CONSTRAINT fk_chunk FOREIGN KEY (chunk_x, chunk_y) 
        REFERENCES chunks(chunk_x, chunk_y) ON DELETE CASCADE
);

-- Index pour performance
CREATE INDEX IF NOT EXISTS idx_buildings_chunk 
    ON buildings(chunk_x, chunk_y);

CREATE INDEX IF NOT EXISTS idx_buildings_coord 
    ON buildings(coord_q, coord_r);

CREATE INDEX IF NOT EXISTS idx_buildings_category 
    ON buildings(category);

CREATE INDEX IF NOT EXISTS idx_buildings_owner 
    ON buildings(owner_id) WHERE owner_id IS NOT NULL;

-- Index GIN pour recherche JSONB
CREATE INDEX IF NOT EXISTS idx_buildings_metadata 
    ON buildings USING GIN (metadata);
-- Create app_configs table
CREATE TABLE IF NOT EXISTS app_configs (
    id SERIAL PRIMARY KEY,
    contract VARCHAR(255) NOT NULL,
    url TEXT NOT NULL,
    uuid VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    billable BOOLEAN,
    ignore_files BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes
CREATE INDEX idx_app_configs_contract ON app_configs(contract);
CREATE INDEX idx_app_configs_uuid ON app_configs(uuid);
CREATE INDEX idx_app_configs_name ON app_configs(name);
CREATE INDEX idx_app_configs_billable ON app_configs(billable);

-- Insert sample data
INSERT INTO app_configs (contract, url, uuid, name, billable, ignore_files) VALUES
('OpenFin / OpenFin', 'http://127.0.0.1:49155/static/manifest-1-jejeut-sweely.json', '1-jhewea-stight', 'app-1-jhewea-stight-longappnamesuffix', NULL, false),
('OpenFin / OpenFin', 'http://127.0.0.1:49155/static/manifest-2-gright-supero.json', '2-ghight-shight', 'app-2-ghight-shight-longappnamesuffix', NULL, false);

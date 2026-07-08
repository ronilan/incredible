import fs from 'fs';
import { execSync } from 'child_process';
import path from 'path';

// Parse Cargo.toml
const cargoToml = fs.readFileSync('Cargo.toml', 'utf8');

// Extract app_name from [package.metadata.bundle]
const metadataMatch = cargoToml.match(/\[package\.metadata\.bundle\][^\[]*app_name\s*=\s\"(.*)\"/);
const appName = metadataMatch ? metadataMatch[1] : 'Incredible';

console.log('Building macOS version for: ' + appName);

// Set APP_NAME env var for the Rust compiler
process.env.APP_NAME = appName;

try {
    // 1. Build the binary
    execSync('cargo build --release --bin incredible_macos --features macos-native', { stdio: 'inherit' });

    // 2. Ensure dist/native directory exists
    fs.mkdirSync(path.join('dist', 'native'), { recursive: true });

    // 3. Copy the binary
    fs.copyFileSync(
        path.join('target', 'release', 'incredible_macos'),
        path.join('dist', 'native', 'incredible_macos')
    );

    console.log('Build successful! Binary copied to dist/native/incredible_macos');
} catch (error) {
    console.error('Build failed:', error.message);
    process.exit(1);
}

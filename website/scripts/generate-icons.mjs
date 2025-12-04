import sharp from 'sharp';
import { readFileSync, writeFileSync } from 'fs';
import { join } from 'path';

const inputSvgPath = join(process.cwd(), '..', 'extension', 'icon.svg');
const outputDirPath = join(process.cwd(), 'public');

const sizes = [180]; // Standard size for apple-touch-icon

async function generateAppleTouchIcons() {
  const svgBuffer = readFileSync(inputSvgPath);

  for (const size of sizes) {
    const outputFileName = `apple-touch-icon.png`;
    const outputPath = join(outputDirPath, outputFileName);

    await sharp(svgBuffer)
      .resize(size, size)
      .toFile(outputPath);

    console.log(`Generated ${outputFileName} (${size}x${size})`);
  }

  // Generate precomposed version (often just a copy in modern usage)
  const precomposedOutputFileName = `apple-touch-icon-precomposed.png`;
  const precomposedOutputPath = join(outputDirPath, precomposedOutputFileName);
  writeFileSync(precomposedOutputPath, readFileSync(join(outputDirPath, `apple-touch-icon.png`)));
  console.log(`Generated ${precomposedOutputFileName}`);
}

generateAppleTouchIcons().catch(err => {
  console.error('Error generating Apple Touch Icons:', err);
  process.exit(1);
});

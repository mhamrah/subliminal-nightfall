#!/usr/bin/env node
/**
 * Generate PNG icon from SVG for VS Code extension
 * 
 * This script requires one of:
 * - sharp: npm install sharp
 * - puppeteer: npm install puppeteer
 * - Or use ImageMagick/Inkscape directly
 */

const fs = require('fs');
const path = require('path');

const svgPath = path.join(__dirname, 'icon.svg');
const pngPath = path.join(__dirname, 'icon.png');

// Try to use sharp if available
try {
  const sharp = require('sharp');
  
  console.log('Using sharp to convert SVG to PNG...');
  sharp(svgPath)
    .resize(512, 512)
    .png()
    .toFile(pngPath)
    .then(() => {
      console.log('✅ icon.png created successfully (512x512)');
    })
    .catch((err) => {
      console.error('Error converting with sharp:', err);
      console.log('\nPlease install sharp: npm install sharp');
      process.exit(1);
    });
} catch (e) {
  console.log('sharp not available. Trying alternative methods...');
  
  // Try puppeteer
  try {
    const puppeteer = require('puppeteer');
    console.log('Using puppeteer to convert SVG to PNG...');
    
    (async () => {
      const browser = await puppeteer.launch();
      const page = await browser.newPage();
      const svgContent = fs.readFileSync(svgPath, 'utf8');
      
      await page.setContent(svgContent);
      await page.setViewport({ width: 512, height: 512 });
      
      const element = await page.$('svg');
      await element.screenshot({ path: pngPath, type: 'png' });
      
      await browser.close();
      console.log('✅ icon.png created successfully (512x512)');
    })();
  } catch (e2) {
    console.error('No conversion library found.');
    console.log('\nPlease install one of:');
    console.log('  npm install sharp');
    console.log('  npm install puppeteer');
    console.log('\nOr use ImageMagick:');
    console.log('  convert -background none -density 512 icon.svg -resize 512x512 icon.png');
    console.log('\nOr use Inkscape:');
    console.log('  inkscape icon.svg --export-filename=icon.png --export-width=512 --export-height=512');
    process.exit(1);
  }
}


#!/usr/bin/env node
import { spawn } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { existsSync } from 'fs';

const __dirname = dirname(fileURLToPath(import.meta.url));

// Check if NLP service is already running
const checkService = async () => {
  try {
    const response = await fetch('http://127.0.0.1:8000/health', { timeout: 2000 });
    return response.ok;
  } catch {
    return false;
  }
};

const startNLP = async () => {
  const isRunning = await checkService();
  
  if (isRunning) {
    console.log('✓ NLP service already running on http://127.0.0.1:8000');
    return;
  }

  console.log('Starting NLP service...');
  
  const nlpPath = join(__dirname, '..', 'src-tauri', 'nlp_service.py');
  
  if (!existsSync(nlpPath)) {
    console.warn(`⚠ Warning: nlp_service.py not found at ${nlpPath}`);
    console.warn('Skipping NLP service startup. Manual start required.');
    return;
  }

  try {
    // Start Python service in background
    const pythonProcess = spawn('python', [nlpPath], {
      detached: true,
      stdio: 'pipe',
      windowsHide: true
    });

    // Unref to allow parent process to exit
    pythonProcess.unref();

    // Give it a moment to start
    await new Promise(resolve => setTimeout(resolve, 2000));

    // Check if it started successfully
    const started = await checkService();
    
    if (started) {
      console.log('✓ NLP service started on http://127.0.0.1:8000');
    } else {
      console.warn('⚠ NLP service may not have started. Check if Python and dependencies are installed.');
      console.warn('  Run: python src-tauri/nlp_service.py');
    }
  } catch (error) {
    console.warn('⚠ Could not start NLP service automatically:', error.message);
    console.warn('  Please run manually: python src-tauri/nlp_service.py');
  }
};

startNLP();

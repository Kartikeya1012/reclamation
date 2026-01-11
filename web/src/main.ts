import './style.css'

const API_URL = 'http://127.0.0.1:3000/api';

interface FileItem {
  path: string;
  reason: string | null;
}

interface TriageResult {
  auto_safe: FileItem[];
  needs_review: FileItem[];
  do_not_touch: FileItem[];
}

interface CleanResponse {
  success: boolean;
  message: string;
  manifest_id: string | null;
}

interface ListResponse {
  manifests: string[];
}

interface SummarizeResponse {
  success: boolean;
  summary: string;
  error?: string;
}

function createApp(): void {
  const app = document.querySelector<HTMLDivElement>('#app')!;
  
  app.innerHTML = `
    <div class="container">
      <h1>Reclamation - Storage Cleanup</h1>
      
      <div class="section">
        <h2>Triage Folder</h2>
        <div class="input-group">
          <input type="text" id="triagePath" placeholder="Enter folder path (e.g., ~/Downloads)" value="~/Downloads">
          <button id="triageBtn">Triage</button>
          <button id="summarizeBtn">Summarize Triage</button>
        </div>
        <div id="summaryResults"></div>
        <div id="triageResults"></div>
      </div>
      
      <div class="section">
        <h2>Clean Folder</h2>
        <div class="input-group">
          <input type="text" id="cleanPath" placeholder="Enter folder path" value="~/Downloads">
          <button id="cleanBtn">Clean</button>
        </div>
        <div id="cleanResults"></div>
      </div>
      
      <div class="section">
        <h2>Manifests</h2>
        <button id="listBtn">List Manifests</button>
        <div id="manifestList"></div>
      </div>
      
      <div class="section">
        <h2>Restore</h2>
        <div class="input-group">
          <input type="text" id="restoreId" placeholder="Manifest ID (leave empty for latest)">
          <button id="restoreBtn">Restore</button>
        </div>
        <div id="restoreResults"></div>
      </div>
    </div>
  `;

  setupEventListeners();
}

function setupEventListeners(): void {
  document.getElementById('triageBtn')?.addEventListener('click', runTriage);
  document.getElementById('summarizeBtn')?.addEventListener('click', runSummarize);
  document.getElementById('cleanBtn')?.addEventListener('click', runClean);
  document.getElementById('listBtn')?.addEventListener('click', listManifests);
  document.getElementById('restoreBtn')?.addEventListener('click', runRestore);
}

async function runTriage(): Promise<void> {
  const path = (document.getElementById('triagePath') as HTMLInputElement).value;
  const results = document.getElementById('triageResults')!;
  results.innerHTML = 'Loading...';

  try {
    const response = await fetch(`${API_URL}/triage/${encodeURIComponent(path)}`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const data: TriageResult = await response.json();

    let html = `<h3>Auto-safe (${data.auto_safe.length} items)</h3>`;
    html += '<div class="file-list auto-safe">';
    data.auto_safe.forEach(item => {
      html += `<div class="file-item">${escapeHtml(item.path)} ${item.reason ? '- ' + escapeHtml(item.reason) : ''}</div>`;
    });
    html += '</div>';

    html += `<h3>Needs Review (${data.needs_review.length} items)</h3>`;
    html += '<div class="file-list needs-review">';
    data.needs_review.forEach(item => {
      html += `<div class="file-item">${escapeHtml(item.path)}</div>`;
    });
    html += '</div>';

    html += `<h3>Do Not Touch (${data.do_not_touch.length} items)</h3>`;
    html += '<div class="file-list do-not-touch">';
    data.do_not_touch.forEach(item => {
      html += `<div class="file-item">${escapeHtml(item.path)}</div>`;
    });
    html += '</div>';

    results.innerHTML = html;
  } catch (error) {
    results.innerHTML = `<div class="error">Error: ${error instanceof Error ? error.message : 'Unknown error'}</div>`;
  }
}

async function runSummarize(): Promise<void> {
  const path = (document.getElementById('triagePath') as HTMLInputElement).value;
  const results = document.getElementById('summaryResults')!;
  results.innerHTML = 'Analyzing files...';

  try {
    const response = await fetch(`${API_URL}/summarize`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ path })
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const data: SummarizeResponse = await response.json();

    if (data.success) {
      results.innerHTML = `<div class="summary">${escapeHtml(data.summary)}</div>`;
    } else {
      results.innerHTML = `<div class="error">${escapeHtml(data.error || 'Unknown error')}</div>`;
    }
  } catch (error) {
    results.innerHTML = `<div class="error">Error: ${error instanceof Error ? error.message : 'Unknown error'}</div>`;
  }
}

async function runClean(): Promise<void> {
  const path = (document.getElementById('cleanPath') as HTMLInputElement).value;
  const results = document.getElementById('cleanResults')!;
  results.innerHTML = 'Cleaning...';
  
  try {
    const response = await fetch(`${API_URL}/clean`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ path })
    });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const data: CleanResponse = await response.json();
    
    if (data.success) {
      results.innerHTML = `<div class="success">${escapeHtml(data.message)}<br>Manifest ID: ${escapeHtml(data.manifest_id || 'N/A')}</div>`;
    } else {
      results.innerHTML = `<div class="error">${escapeHtml(data.message)}</div>`;
    }
  } catch (error) {
    results.innerHTML = `<div class="error">Error: ${error instanceof Error ? error.message : 'Unknown error'}</div>`;
  }
}

async function listManifests(): Promise<void> {
  const results = document.getElementById('manifestList')!;
  results.innerHTML = 'Loading...';
  
  try {
    const response = await fetch(`${API_URL}/list`);
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const data: ListResponse = await response.json();
    
    if (data.manifests.length === 0) {
      results.innerHTML = '<div>No manifests found</div>';
    } else {
      let html = '<div class="file-list">';
      data.manifests.forEach(id => {
        html += `<div class="file-item">${escapeHtml(id)} <button onclick="restoreById('${escapeHtml(id)}')">Restore</button></div>`;
      });
      html += '</div>';
      results.innerHTML = html;
    }
  } catch (error) {
    results.innerHTML = `<div class="error">Error: ${error instanceof Error ? error.message : 'Unknown error'}</div>`;
  }
}

async function runRestore(): Promise<void> {
  const id = (document.getElementById('restoreId') as HTMLInputElement).value;
  const results = document.getElementById('restoreResults')!;
  results.innerHTML = 'Restoring...';
  
  try {
    let manifestId = id;
    if (!manifestId) {
      const listResponse = await fetch(`${API_URL}/list`);
      if (!listResponse.ok) throw new Error('Failed to list manifests');
      const listData: ListResponse = await listResponse.json();
      if (listData.manifests.length === 0) throw new Error('No manifests found');
      manifestId = listData.manifests[listData.manifests.length - 1];
    }
    
    const response = await fetch(`${API_URL}/restore/${manifestId}`, { method: 'POST' });
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    const data: CleanResponse = await response.json();
    
    if (data.success) {
      results.innerHTML = `<div class="success">${escapeHtml(data.message)}</div>`;
    } else {
      results.innerHTML = `<div class="error">${escapeHtml(data.message)}</div>`;
    }
  } catch (error) {
    results.innerHTML = `<div class="error">Error: ${error instanceof Error ? error.message : 'Unknown error'}</div>`;
  }
}

function restoreById(id: string): void {
  (document.getElementById('restoreId') as HTMLInputElement).value = id;
  runRestore();
}

function escapeHtml(text: string): string {
  const div = document.createElement('div');
  div.textContent = text;
  return div.innerHTML;
}

// Make restoreById available globally for onclick handlers
(window as any).restoreById = restoreById;

createApp();

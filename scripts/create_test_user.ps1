$ErrorActionPreference = "Stop"

Write-Host "⏳ Waiting for API to be ready..."

# Retry loop to wait for API
$maxRetries = 30
$retryCount = 0
$apiUrl = "http://localhost:3000/api/auth/register"

$body = @{
    username = "admin"
    password = "password"
    name = "Admin User"
    role = "MANAGER"
} | ConvertTo-Json

while ($retryCount -lt $maxRetries) {
    try {
        $response = Invoke-RestMethod -Uri $apiUrl -Method Post -Body $body -ContentType "application/json"
        Write-Host "✅ User created successfully!"
        Write-Host "Username: admin"
        Write-Host "Password: password"
        exit 0
    } catch {
        $status = $_.Exception.Response.StatusCode.value__
        if ($status -eq 500 -or $status -eq 409) {
            # 409 Conflict likely means user exists (if we handled it), but here 500 is common for DB dupes in our code
            Write-Host "⚠️  User might already exist or server error. Trying to Login to check..."
            
            try {
                $loginBody = @{ username = "admin"; password = "password" } | ConvertTo-Json
                $login = Invoke-RestMethod -Uri "http://localhost:3000/api/auth/login" -Method Post -Body $loginBody -ContentType "application/json"
                Write-Host "✅ User 'admin' already exists and login works."
                exit 0
            } catch {
                Write-Host "❌ Failed to create user and failed to check login."
                Write-Host $_.Exception.Message
            }
        }
        
        Write-Host "⏳ API not ready yet (Attempt $($retryCount + 1)/$maxRetries). Retrying in 5s..."
        Start-Sleep -Seconds 5
        $retryCount++
    }
}

Write-Host "❌ Timed out waiting for API."
exit 1

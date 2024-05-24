def longest_slide_down(pyramid):  
    dp = [row[:] for row in pyramid]   
    for row in range(len(dp) - 2, -1, -1):
        for col in range(len(dp[row])):           
            dp[row][col] += max(dp[row + 1][col], dp[row + 1][col + 1])
    
    
    return dp[0][0]
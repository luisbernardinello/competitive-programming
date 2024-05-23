def power_of_two_greater_than(n):
    power = 1
    while power < n:
        power <<= 1
    return power

def sum_range(start, end):
    return (start + end) * (end - start + 1) // 2

def elder_age(height, width, loss, limit):
    if height == 0 or width == 0:
        return 0
    
    if height > width:
        height, width = width, height

    next_power_height = power_of_two_greater_than(height)
    next_power_width = power_of_two_greater_than(width)

    if loss > next_power_width:
        return 0

    if next_power_height == next_power_width:
        total_time = (sum_range(1, next_power_width - loss - 1) * (height + width - next_power_width) 
                      + elder_age(next_power_width - width, next_power_height - height, loss, limit)) % limit
        return total_time
    
    if next_power_height < next_power_width:
        next_power_height = next_power_width // 2
        temp_sum = (sum_range(1, next_power_width - loss - 1) * height 
                    - (next_power_width - width) * sum_range(max(0, next_power_height - loss), next_power_width - loss - 1))
        
        if loss <= next_power_height:
            temp_sum += ((next_power_height - loss) * (next_power_height - height) * (next_power_width - width) 
                         + elder_age(next_power_height - height, next_power_width - width, 0, limit))
        else:
            temp_sum += elder_age(next_power_height - height, next_power_width - width, loss - next_power_height, limit)
        
        return temp_sum % limit
# To run the app:
    first go to diet_manager_gui directrix
    then type cargo build
    then cargo run


# Diet Manager - Feature Exercise Guide

## 1. User Authentication
### Login:
1. Launch the application
2. Enter registered username and password
3. Click "Login" button
4. Verify successful login shows username in top bar

### Registration:
1. From login screen, click "Register"
2. Fill in:
   - Username
   - Password
   - Profile details (age, weight, height, activity level)
3. Click "Register"
4. Verify you can login with new credentials

## 2. Home Screen Features
### Navigation:
- Click any button to navigate to corresponding page:
  - "Add Basic Food"
  - "Add Composite Food"
  - "View Daily Log"
  - "Add Food to Log"
  - "Edit Food Log"
  - "Update Profile"
  - "Download Food Data"

### Daily Nutrition Overview:
- Verify displays:
  - Current date
  - Calories consumed today
  - Daily calorie goal
  - Remaining/exceeded calories
  - Progress bar showing percentage of goal

### Undo Functionality:
1. Perform any action (e.g., add food)
2. Click "Undo" button in bottom right
3. Verify action is reversed
4. Check notification shows what was undone

## 3. Basic Food Management
### Add Basic Food:
1. Navigate to "Add Basic Food"
2. Fill in:
   - Name (e.g., "Apple")
   - Identifier (e.g., "FR001")
   - Calories per serving
   - Serving size/unit
3. Click "Add Food"
4. Verify appears in food lists

## 4. Composite Food Management
### Add Composite Food:
1. Navigate to "Add Composite Food"
2. Enter name and identifier
3. Add ingredients from existing foods
4. Set quantities for each ingredient
5. Click "Create Composite Food"
6. Verify appears in food lists

## 5. Daily Log Features
### View Daily Log:
1. Navigate to "View Daily Log"
2. Verify shows current day's foods
3. Use date navigation to view different days
4. Test:
   - Previous/Next day buttons
   - Future dates should be disabled

### Remove Food Entry:
1. Find food entry in daily log
2. Click "❌" button
3. Verify entry disappears
4. Use Undo button to restore

## 6. Add Food to Log
### Add Food:
1. Navigate to "Add Food to Log"
2. Search for food by:
   - Name (partial matches)
   - Identifier
3. Select food
4. Adjust serving size
5. Click "Add to Log"
6. Verify appears in daily log

## 7. Edit Food Log
### Modify Entries:
1. Navigate to "Edit Food Log"
2. For any entry:
   - Adjust serving size and click "Update"
   - Or click "Remove" to delete
3. Verify changes reflect in daily log

## 8. Profile Management
### Update Profile:
1. Navigate to "Update Profile"
2. Modify any fields:
   - Personal details
   - Weight/height
   - Activity level
   - Calorie goal preferences
3. Click "Update Profile"
4. Verify daily calorie goal updates

## New Feature: Download Food Data Page
### Access External Databases:
1. Navigate to "Download Food Data" from home screen
2. Select data source from options:
   - McDonald's
   - USDA
   - MyFitnessPal
   - Custom URL

### For Standard Sources:
1. Select McDonald's/USDA/MyFitnessPal
2. Click "Download" button
3. Verify message appears: "Downloading from [Source]... (Not implemented yet)"
4. (Future implementation will show parsed data preview)

### For Custom Sources:
1. Select "Custom" option
2. Enter food data URL (e.g., restaurant nutrition page)
3. Click "Download"
4. Verify:
   - Empty URL shows error
   - Valid URL shows "Downloading from [URL]... (Not implemented yet)"

### Expected Future Behavior:
- Will display parsed food items
- Allow saving to local database
- Show download progress
- Handle API errors gracefully

### JSON File Structure:
- foods.json:
  - Contains basic and composite foods
  - Structure: {id, name, calories, ingredients (for composites)}
  
- logs.json:
  - Contains daily food entries
  - Structure: {date, user_id, food_id, servings}

- users.json:
  - Contains user profiles
  - Structure: {user_id, username, password_hash, profile_data}





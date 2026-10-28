#![allow(warnings)]
use std::io;

fn printGrid(grid: [[i32; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            if grid[i][j] != -1 {
                print!("{}", grid[i][j]);
            }
            else {
                print!(" ");
            }
            if j != 2 {
                print!("  |");
            }
        }
        println!("");
        if i != 2 {
            for k in 0..3 {
                print!("----");
            }
            println!("");
        }
    }
}

fn winChecker(grid: [[i32; 3]; 3]) -> bool {
    if grid[0][0] == grid[0][1] && grid[0][1] == grid[0][2] && grid[0][0] != -1 {
        return true;
    }
    else if grid[1][0] == grid[1][1] && grid[1][1] == grid[1][2] && grid[1][0] != -1 {
        return true;
    } 
    else if grid[2][0] == grid[2][1] && grid[2][1] == grid[2][2] && grid[2][0] != -1 {
        return true;
    } 
    else if grid[0][0] == grid[1][0] && grid[1][0] == grid[2][0] && grid[0][0] != -1 {
        return true;
    } 
    else if grid[0][1] == grid[1][1] && grid[1][1] == grid[2][1] && grid[0][1] != -1 {
        return true;
    } 
    else if grid[0][2] == grid[1][2] && grid[1][2] == grid[2][2] && grid[0][2] != -1 {
        return true;
    } 
    else if grid[0][0] == grid[1][1] && grid[1][1] == grid[2][2] && grid[0][0] != -1 {
        return true;
    } 
    else if grid[0][2] == grid[1][1] && grid[1][1] == grid[2][0] && grid[0][2] != -1 {
        return true;
    }
    else {
        return false;
    }  
}

fn drawChecker(grid: [[i32; 3]; 3]) -> bool {
    for i in 0..3 {
        for j in 0..3 {
            if grid[i][j] == -1 {
                return false;
            }
        }
    }
    return true;
}

fn spaceChecker(cell: i32, grid: [[i32; 3]; 3]) -> bool {
    if cell == 0 {
        if grid[0][0] != -1 {
            return false;
        }
        else {
            return true;
        }
    }
    else if cell == 1 {
        if grid[0][1] != -1 {
            return false;
        }
        else {
            return true;
        }
    }
    else if cell == 2 {
        if grid[0][2] != -1 {
            return false;
        }
        else {
            return true;
        }
    }
    else if cell == 3 {
        if grid[1][0] != -1 {
            return false;
        }
        else {
            return true;
        }
    }
    else if cell == 4 {
        if grid[1][1] != -1 {
            return false;
        }
        else {
            return true;
        }
    }
    else if cell == 5 {
        if grid[1][2] != -1 {
            return false;
        }
        else {
            return true;
        }
    }
    else if cell == 6 {
        if grid[2][0] != -1 {
            return false;
        }
        else {
            return true;
        }
    }
    else if cell == 7 {
        if grid[2][1] != -1 {
            return false;
        }
        else {
            return true;
        }
    }
    else {
        if grid[2][2] != -1 {
            return false;
        }
        else {
            return true;
        }
    }
}

fn takeTurn(mut player: i32, cell: i32, mut grid: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    if cell == 0 {
        grid[0][0] = player;
    }
    else if cell == 1 {
        grid[0][1] = player;
    }
    else if cell == 2 {
        grid[0][2] = player;
    }
    else if cell == 3 {
        grid[1][0] = player;
    }
    else if cell == 4 {
        grid[1][1] = player;
    }
    else if cell == 5 {
        grid[1][2] = player;
    }
    else if cell == 6 {
        grid[2][0] = player;
    }
    else if cell == 7 {
        grid[2][1] = player;
    }
    else {
        grid[2][2] = player;
    }
    return grid;
}

fn startGame() -> io::Result<()> {
    let mut grid: [[i32; 3]; 3] = [[-1; 3]; 3];
    let mut player = 1;
    let stdin = io::stdin();
    while true {
        if player == 1 {
            let mut mov = String::new();
            println!("Player 1's turn. Enter Cell Number from 1-9");
            printGrid(grid);
            stdin.read_line(&mut mov);
            let cell: i32 = mov[0..1].parse().unwrap();
            let cell = cell - 1;
            if spaceChecker(cell, grid) {
                grid = takeTurn(player, cell, grid);
                if winChecker(grid) {
                    println!("Player 1 has won the game!");
                    printGrid(grid);
                    break;
                }
                else if drawChecker(grid) {
                    println!("The game is a draw");
                    printGrid(grid);
                    break;
                }
                player = 2;
            }
            else {
                println!("No space available. Enter a valid cell number");
                continue;
            }
        }
        else if player == 2 {
            let mut mov = String::new();
            println!("Player 2's turn. Enter Cell Number from 1-9");
            printGrid(grid);
            stdin.read_line(&mut mov);
            let cell: i32 = mov[0..1].parse().unwrap();
            let cell = cell - 1;
            if spaceChecker(cell, grid) {
                grid = takeTurn(player, cell, grid);
                if winChecker(grid) {
                    println!("Player 2 has won the game!");
                    printGrid(grid);
                    break;
                }
                else if drawChecker(grid) {
                    println!("The game is a draw");
                    printGrid(grid);
                    break;
                }
                player = 1;
            }
            else {
                println!("No space available. Enter a valid cell number");
                continue;
            }
        }
    }
    return Ok(());
}
fn main() {
    startGame();
}

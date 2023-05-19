using GridSizeType = unsigned int;
using CellCoordinateType = unsigned int;
using CellSizeType = unsigned int;
using FloatType = float;


#include <iostream>
#include <fstream>
#include <filesystem>
#include <json/json.h>
#include "../grid/NonUniformGrid.hpp"
#include "../configuration/Configuration.hpp"


using CellType = GridCell<CellCoordinateType, CellSizeType>;


void process(
        const Configuration<GridSizeType, CellSizeType> &_config,
        NonUniformGrid<GridSizeType, CellCoordinateType, CellSizeType> &_grid
) {
    if (_grid.getSy() == 0 || _grid.getSx() == 0) {
        return;
    }
    const auto _x0 = _grid.getX0();
    const auto _y0 = _grid.getY0();
    auto current_x = _x0;
    auto current_y = _y0;

    const auto sx = _config.getSx();
    const auto sy = _config.getSy();

    const auto minSx = _config.getMinSx();
    const auto minSy = _config.getMinSy();

    const auto maxSx = _config.getMaxSx();
    const auto maxSy = _config.getMaxSy();

    // Bottom border
    while (current_x < sx) {
        if (current_x + maxSx <= sx) {
            _grid.addGridCell(CellType(current_x, current_y, maxSx, maxSy));
            current_x += maxSx;
        } else {
            if (current_x + minSx <= sx) {
                _grid.addGridCell(CellType(current_x, current_y, minSx, maxSy));
                current_x += minSx;
            } else {
                std::cerr << "No way to add cell!\nCurrent x = " << current_x;
                current_x = sx;
            }
        }
    }

    // Right border
    while (current_y < sy) {
        if (current_x + maxSx <= sx) {
            _grid.addGridCell(CellType(current_x, current_y, maxSx, maxSy));
            current_x += maxSx;
        } else {
            if (current_x + minSx <= sx) {
                _grid.addGridCell(CellType(current_x, current_y, minSx, maxSy));
                current_x += minSx;
            } else {
                std::cerr << "No way to add cell!\nCurrent x = " << current_x;
                current_x = sx;
            }
        }
    }

}


int main(int argc, char **argv) {
    if (argc != 2) {
        return -1;
    }
    const auto _json_file_path = std::string(argv[1]);
    const auto config = read_json(_json_file_path);

    NonUniformGrid<GridSizeType, CellCoordinateType, CellSizeType> grid(config.getSx(), config.getSy());
    process(config, grid);
    const auto inner = grid.innerShape();
    std::cout << inner.getX0() << ' ' << inner.getY0() << ' ' << inner.getSx() << ' ' << inner.getSy() << '\n';
    std::cout << "Square: " << grid.square() << '\n';

    return 0;
}
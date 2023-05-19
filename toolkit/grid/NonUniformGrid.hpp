//
// Created by a.pesterev on 18.04.2023.
//

#ifndef PACKDESIGNER_NONUNIFORMGRID_HPP
#define PACKDESIGNER_NONUNIFORMGRID_HPP


#include <algorithm>
#include <list>
#include "GridCell.hpp"

constexpr FloatType normal_y = FloatType(1.);
constexpr FloatType dt = FloatType(0.01);


template<typename _coordinate_type>
class BreakPoint {
public:

    BreakPoint() = delete;

    explicit BreakPoint(_coordinate_type x0, _coordinate_type y0) : x0(x0), y0(y0) {}

private:

    _coordinate_type x0;
    _coordinate_type y0;

public:

    _coordinate_type getX0() const {
        return x0;
    }

    _coordinate_type getY0() const {
        return y0;
    }

};


template<typename _coordinate_type, typename _cell_size_type>
struct byRadiusVectorTillBegin {
    _coordinate_type _gx0;
    _coordinate_type _gy0;

    byRadiusVectorTillBegin(_coordinate_type gx0, _coordinate_type gy0) : _gx0(gx0), _gy0(gy0) {}

    bool operator()(const GridCell<_coordinate_type, _cell_size_type> &_cellA,
                    const GridCell<_coordinate_type, _cell_size_type> &_cellB) {
        const auto f00 = _gx0 + _cellA.getX0();
        const auto f01 = _gy0 + _cellA.getY0();

        const auto f10 = _gx0 + _cellB.getX0();
        const auto f11 = _gy0 + _cellB.getY0();

        return ((f00 * f00) + (f01 * f01)) < ((f10 * f10) + (f11 * f11));
    }
};


template<typename _size_type, typename _coordinate_type, typename _cell_size_type>
class NonUniformGrid {

    using GridType = NonUniformGrid<_size_type, _coordinate_type, _cell_size_type>;
    using CellType = GridCell<_coordinate_type, _cell_size_type>;

public:

    NonUniformGrid() = delete;

    explicit NonUniformGrid(_size_type sx, _size_type sy) : sx(sx), sy(sy) {}

    explicit NonUniformGrid(_coordinate_type x0, _coordinate_type y0, _size_type sx, _size_type sy) : x0(x0), y0(y0),
                                                                                                      sx(sx),
                                                                                                      sy(sy) {}

private:

    _coordinate_type x0 = _coordinate_type(0);
    _coordinate_type y0 = _coordinate_type(0);
    _size_type sx;
    _size_type sy;
    std::vector<CellType> cells;

public:

    _coordinate_type getX0() const {
        return x0;
    }

    _coordinate_type getY0() const {
        return y0;
    }

    _size_type getSx() const {
        return sx;
    }

    _size_type getSy() const {
        return sy;
    }

public:

    void addGridCell(const CellType &_cell) {
        cells.push_back(_cell);
    }

    bool pointIsFree(const _coordinate_type &_x0, const _coordinate_type &_y0) const {
        for (const auto &_cell: cells) {
            if (_cell.contains(_x0, _y0)) {
                return false;
            }
        }
        return true;
    }

    GridType innerShape() {
        std::sort(cells.begin(), cells.end(), [](const GridCell<CellCoordinateType, CellSizeType> &_c1,
                                                 const GridCell<CellCoordinateType, CellSizeType> &_c2) {
            const auto _y0 = _c1.getY0();
            const auto _y1 = _c2.getY0();
            const auto _x0 = _c1.getX0();
            const auto _x1 = _c2.getX0();
            if (_y0 == _y1) {
                return _x0 < _x1;
            }
            return _y0 < _y1;
        });

        for (const auto &_cell: cells) {
            std::cout << "Cell: " << _cell.getX0() << ' ' << _cell.getY0() << '\n';
        }

        const auto cellBottomLeft = cells[0];
        const auto cellTopRight = cells[cells.size() - 1];

        const auto _inner_x0 = cellBottomLeft.getX0() + cellBottomLeft.getSx();
        const auto _inner_y0 = cellBottomLeft.getY0() + cellBottomLeft.getSy();

        const auto _inner_x1 = cellTopRight.getX0();
        const auto _inner_y1 = cellTopRight.getY0();

        std::cout << _inner_x0 << ' ' << _inner_y0 << '\n';
        std::cout << _inner_x1 << ' ' << _inner_y1 << '\n';


        auto _inner_sx = _inner_x1 - _inner_x0;
        auto _inner_sy = _inner_y1 - _inner_y0;

        if (_inner_x1 < _inner_x0) {
            _inner_sx = 0;
        }

        if (_inner_y1 < _inner_y0) {
            _inner_sy = 0;
        }

        return GridType(
                _inner_x0,
                _inner_y0,
                _inner_sx,
                _inner_sy
        );
    }

    _cell_size_type square() const {
        auto _square = (_cell_size_type) 0;
        for (const auto &_cell: cells) {
            _square += _cell.square();
        }
        return _square;
    }

    std::vector<CellType> getOffsetCells() const {
        std::vector<CellType> _result;
        for (const auto& _cell: cells) {
            CellType copy = _cell;
            copy.setX0(copy.getX0() + x0);
            copy.setY0(copy.getY0() + y0);
            _result.push_back(copy);
        }
        return _result;
    }

    void addCells(const std::vector<CellType>& _cells) {
        for (const auto& cell: _cells) {
            cells.push_back(cell);
        }
    }

};


#endif //PACKDESIGNER_NONUNIFORMGRID_HPP

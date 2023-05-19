//
// Created by a.pesterev on 18.04.2023.
//

#ifndef PACKDESIGNER_GRIDCELL_HPP
#define PACKDESIGNER_GRIDCELL_HPP

template <typename _coordinate_type, typename _size_type>
class GridCell {
public:

    GridCell() = delete;

    explicit GridCell(_coordinate_type x0, _coordinate_type y0, _size_type sx, _size_type sy) : x0(x0), y0(y0), sx(sx), sy(sy) {}

private:

    _coordinate_type x0;
    _coordinate_type y0;

    _size_type sx;
    _size_type sy;

public:

    void setX0(_coordinate_type _x0) {
        x0 = _x0;
    }

    void setY0(_coordinate_type _y0) {
        y0 = _y0;
    }

    void setSx(_size_type _sx) {
        sx = _sx;
    }

    void setSy(_size_type _sy) {
        sy = _sy;
    }

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

    // Contains - not on border
    bool contains(const _coordinate_type& _x0, const _coordinate_type& _y0) const {
        const auto xc0 = this->x0;
        const auto yc0 = this->y0;
        const auto sizeX = this->sx;
        const auto sizeY = this->sy;
        const auto next_x_border = xc0 + sizeX;
        const auto next_y_border = yc0 + sizeY;
        if (xc0 == _x0 || next_x_border == _x0) {
            if (yc0 <= _y0 && next_y_border >= _y0) {
                return false;
            }
        }
        if (yc0 == _y0 || next_y_border == _y0) {
            if (xc0 <= _x0 && next_x_border >= _x0) {
                return false;
            }
        }
        if (xc0 < _x0 && next_x_border > _x0) {
            if (yc0 < _y0 && next_y_border > _y0) {
                return true;
            }
        }
        return false;
    }

    _size_type square() const {
        const auto sizeX = this->sx;
        const auto sizeY = this->sy;
        return sizeX * sizeY;
    }

public:

    _coordinate_type getRightBorderX() const {
        return x0 + sx;
    }

    _coordinate_type getTopBorderY() const {
        return y0 + sy;
    }

};


#endif //PACKDESIGNER_GRIDCELL_HPP

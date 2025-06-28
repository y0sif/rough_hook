/****************************************************************************
** Meta object code from reading C++ file 'timecontrolwidget.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/timecontrolwidget.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'timecontrolwidget.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_TimeControlWidget_t {
    QByteArrayData data[11];
    char stringdata0[143];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_TimeControlWidget_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_TimeControlWidget_t qt_meta_stringdata_TimeControlWidget = {
    {
QT_MOC_LITERAL(0, 0, 17), // "TimeControlWidget"
QT_MOC_LITERAL(1, 18, 22), // "timeControlModeChanged"
QT_MOC_LITERAL(2, 41, 0), // ""
QT_MOC_LITERAL(3, 42, 4), // "Mode"
QT_MOC_LITERAL(4, 47, 4), // "mode"
QT_MOC_LITERAL(5, 52, 18), // "setTimeControlMode"
QT_MOC_LITERAL(6, 71, 29), // "onOtherTimeControlModeChanged"
QT_MOC_LITERAL(7, 101, 10), // "Tournament"
QT_MOC_LITERAL(8, 112, 11), // "TimePerMove"
QT_MOC_LITERAL(9, 124, 8), // "Infinite"
QT_MOC_LITERAL(10, 133, 9) // "Hourglass"

    },
    "TimeControlWidget\0timeControlModeChanged\0"
    "\0Mode\0mode\0setTimeControlMode\0"
    "onOtherTimeControlModeChanged\0Tournament\0"
    "TimePerMove\0Infinite\0Hourglass"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_TimeControlWidget[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       3,   14, // methods
       0,    0, // properties
       1,   38, // enums/sets
       0,    0, // constructors
       0,       // flags
       1,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    1,   29,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
       5,    1,   32,    2, 0x0a /* Public */,
       6,    1,   35,    2, 0x08 /* Private */,

 // signals: parameters
    QMetaType::Void, 0x80000000 | 3,    4,

 // slots: parameters
    QMetaType::Void, 0x80000000 | 3,    4,
    QMetaType::Void, 0x80000000 | 3,    4,

 // enums: name, alias, flags, count, data
       3,    3, 0x0,    4,   43,

 // enum data: key, value
       7, uint(TimeControlWidget::Tournament),
       8, uint(TimeControlWidget::TimePerMove),
       9, uint(TimeControlWidget::Infinite),
      10, uint(TimeControlWidget::Hourglass),

       0        // eod
};

void TimeControlWidget::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<TimeControlWidget *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->timeControlModeChanged((*reinterpret_cast< Mode(*)>(_a[1]))); break;
        case 1: _t->setTimeControlMode((*reinterpret_cast< Mode(*)>(_a[1]))); break;
        case 2: _t->onOtherTimeControlModeChanged((*reinterpret_cast< Mode(*)>(_a[1]))); break;
        default: ;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (TimeControlWidget::*)(Mode );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&TimeControlWidget::timeControlModeChanged)) {
                *result = 0;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject TimeControlWidget::staticMetaObject = { {
    QMetaObject::SuperData::link<QWidget::staticMetaObject>(),
    qt_meta_stringdata_TimeControlWidget.data,
    qt_meta_data_TimeControlWidget,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *TimeControlWidget::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *TimeControlWidget::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_TimeControlWidget.stringdata0))
        return static_cast<void*>(this);
    return QWidget::qt_metacast(_clname);
}

int TimeControlWidget::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QWidget::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 3)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 3;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 3)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 3;
    }
    return _id;
}

// SIGNAL 0
void TimeControlWidget::timeControlModeChanged(Mode _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 0, _a);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE

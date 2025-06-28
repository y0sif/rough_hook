/****************************************************************************
** Meta object code from reading C++ file 'gameviewer.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/gameviewer.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'gameviewer.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_GameViewer_t {
    QByteArrayData data[17];
    char stringdata0[216];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_GameViewer_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_GameViewer_t qt_meta_stringdata_GameViewer = {
    {
QT_MOC_LITERAL(0, 0, 10), // "GameViewer"
QT_MOC_LITERAL(1, 11, 12), // "moveSelected"
QT_MOC_LITERAL(2, 24, 0), // ""
QT_MOC_LITERAL(3, 25, 10), // "moveNumber"
QT_MOC_LITERAL(4, 36, 8), // "viewMove"
QT_MOC_LITERAL(5, 45, 5), // "index"
QT_MOC_LITERAL(6, 51, 7), // "keyLeft"
QT_MOC_LITERAL(7, 59, 20), // "viewFirstMoveClicked"
QT_MOC_LITERAL(8, 80, 23), // "viewPreviousMoveClicked"
QT_MOC_LITERAL(9, 104, 19), // "viewNextMoveClicked"
QT_MOC_LITERAL(10, 124, 19), // "viewLastMoveClicked"
QT_MOC_LITERAL(11, 144, 19), // "viewPositionClicked"
QT_MOC_LITERAL(12, 164, 12), // "onFenChanged"
QT_MOC_LITERAL(13, 177, 3), // "fen"
QT_MOC_LITERAL(14, 181, 10), // "onMoveMade"
QT_MOC_LITERAL(15, 192, 18), // "Chess::GenericMove"
QT_MOC_LITERAL(16, 211, 4) // "move"

    },
    "GameViewer\0moveSelected\0\0moveNumber\0"
    "viewMove\0index\0keyLeft\0viewFirstMoveClicked\0"
    "viewPreviousMoveClicked\0viewNextMoveClicked\0"
    "viewLastMoveClicked\0viewPositionClicked\0"
    "onFenChanged\0fen\0onMoveMade\0"
    "Chess::GenericMove\0move"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_GameViewer[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
      10,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       1,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    1,   64,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
       4,    2,   67,    2, 0x0a /* Public */,
       4,    1,   72,    2, 0x2a /* Public | MethodCloned */,
       7,    0,   75,    2, 0x08 /* Private */,
       8,    0,   76,    2, 0x08 /* Private */,
       9,    0,   77,    2, 0x08 /* Private */,
      10,    0,   78,    2, 0x08 /* Private */,
      11,    1,   79,    2, 0x08 /* Private */,
      12,    1,   82,    2, 0x08 /* Private */,
      14,    1,   85,    2, 0x08 /* Private */,

 // signals: parameters
    QMetaType::Void, QMetaType::Int,    3,

 // slots: parameters
    QMetaType::Void, QMetaType::Int, QMetaType::Bool,    5,    6,
    QMetaType::Void, QMetaType::Int,    5,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void, QMetaType::Int,    5,
    QMetaType::Void, QMetaType::QString,   13,
    QMetaType::Void, 0x80000000 | 15,   16,

       0        // eod
};

void GameViewer::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<GameViewer *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->moveSelected((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 1: _t->viewMove((*reinterpret_cast< int(*)>(_a[1])),(*reinterpret_cast< bool(*)>(_a[2]))); break;
        case 2: _t->viewMove((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 3: _t->viewFirstMoveClicked(); break;
        case 4: _t->viewPreviousMoveClicked(); break;
        case 5: _t->viewNextMoveClicked(); break;
        case 6: _t->viewLastMoveClicked(); break;
        case 7: _t->viewPositionClicked((*reinterpret_cast< int(*)>(_a[1]))); break;
        case 8: _t->onFenChanged((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 9: _t->onMoveMade((*reinterpret_cast< const Chess::GenericMove(*)>(_a[1]))); break;
        default: ;
        }
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        switch (_id) {
        default: *reinterpret_cast<int*>(_a[0]) = -1; break;
        case 9:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< Chess::GenericMove >(); break;
            }
            break;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (GameViewer::*)(int );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&GameViewer::moveSelected)) {
                *result = 0;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject GameViewer::staticMetaObject = { {
    QMetaObject::SuperData::link<QWidget::staticMetaObject>(),
    qt_meta_stringdata_GameViewer.data,
    qt_meta_data_GameViewer,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *GameViewer::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *GameViewer::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_GameViewer.stringdata0))
        return static_cast<void*>(this);
    return QWidget::qt_metacast(_clname);
}

int GameViewer::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QWidget::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 10)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 10;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 10)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 10;
    }
    return _id;
}

// SIGNAL 0
void GameViewer::moveSelected(int _t1)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))) };
    QMetaObject::activate(this, &staticMetaObject, 0, _a);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
